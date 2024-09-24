#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use enigo::{Enigo, MouseControllable};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, GlobalShortcutManager, Manager, State};

#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;
#[cfg(target_os = "windows")]
use winapi::um::winuser::{SetWindowLongPtrW, GWLP_WNDPROC};

#[cfg(target_os = "windows")]
mod mouse_tracker;
#[cfg(target_os = "windows")]
use mouse_tracker::{AppState, MouseTracker, APP_STATE};

#[cfg(not(target_os = "windows"))]
mod mouse_tracker_mock {
    use once_cell::sync::Lazy;
    use std::sync::{Arc, Mutex};

    pub struct MouseTracker {
        pub tracking: bool,
        pub count: i32,
    }

    impl MouseTracker {
        pub fn new() -> Self {
            MouseTracker {
                tracking: false,
                count: 0,
            }
        }

        pub fn start_tracking(&mut self) -> Result<(), String> {
            println!("Mock start tracking on macOS");
            self.tracking = true;
            Ok(())
        }

        pub fn stop_tracking(&mut self) -> Result<(), String> {
            println!("Mock stop tracking on macOS");
            self.tracking = false;
            Ok(())
        }
    }

    pub struct AppState {
        pub current_page: String,
        pub tracker: MouseTracker,
    }

    pub static APP_STATE: Lazy<Mutex<Option<Arc<Mutex<AppState>>>>> = Lazy::new(|| Mutex::new(None));
}
#[cfg(not(target_os = "windows"))]
use mouse_tracker_mock::{AppState, MouseTracker, APP_STATE};

mod calculations;
mod utils;
mod commands;
mod models;

use calculations::{calculate_counts, calculate_scoped_counts, calculate_yaw, estimate_fov};
use crate::commands::{get_app_settings, get_games, get_initial_values, get_yaw_values, save_game_yaw, set_app_settings, set_current_page, set_user_settings, set_yaw_values};
use crate::models::{UserSettings, YawStuff};
use crate::utils::{load_app_settings, setup_global_shortcuts};

fn main() {
    let app_settings = load_app_settings().expect("Failed to load settings");
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(UserSettings {
            cm360: 0.0,
            dpi: 0,
            normal_fov: 0.0,
            scoped_fov: 0.0,
            game_sens: 0.0,
            game_fov: 0.0,
        })))
        .manage(Arc::new(Mutex::new(AppState {
            current_page: "main_sensitivity".to_string(),
            tracker: MouseTracker::new(),
        })))
        .manage(Arc::new(Mutex::new(app_settings)))
        .manage(Arc::new(Mutex::new(YawStuff {
            sens: 1.0,
            counts: 0,
            inc: 0.0,
            yaw: 0.0,
            lower_limit: 0.0,
            upper_limit: 1000.0,
        })))
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                let window = app.get_window("main").unwrap();
                let hwnd = match window.hwnd() {
                    Ok(hwnd) => hwnd.0 as HWND,
                    Err(_) => panic!("Failed to get window handle"),
                };

                let app_state: State<Arc<Mutex<AppState>>> = app.state();
                *APP_STATE.lock().unwrap() = Some(app_state.inner().clone());

                unsafe {
                    SetWindowLongPtrW(hwnd, GWLP_WNDPROC, MouseTracker::window_proc as isize);
                }
            }

            #[cfg(not(target_os = "windows"))]
            {
                let app_state: State<Arc<Mutex<AppState>>> = app.state();
                *APP_STATE.lock().unwrap() = Some(app_state.inner().clone());
            }

            setup_global_shortcuts(app.handle());

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                event.window().app_handle().exit(0);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            set_user_settings,
            set_current_page,
            get_initial_values,
            set_app_settings,
            get_app_settings,
            set_yaw_values,
            get_yaw_values,
            save_game_yaw,
            get_games
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

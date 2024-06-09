#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::{GlobalShortcutManager, Manager, State, AppHandle};
use enigo::{Enigo, MouseControllable};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{SetWindowLongPtrW, GWLP_WNDPROC};
use serde::Serialize;

mod mouse_tracker;
use mouse_tracker::{MouseTracker, AppState, APP_STATE};

mod calculations;
use calculations::{calculate_scoped_counts, calculate_yaw, estimate_fov, calculate_counts};

#[derive(Serialize)]
struct UserSettings {
    cm360: f32,
    dpi: i32,
    normal_fov: f32,
    scoped_fov: f32,
}

#[tauri::command]
fn set_user_settings(cm360: f32, dpi: i32, normal_fov: f32, scoped_fov: f32, state: State<'_, Arc<Mutex<UserSettings>>>) {
    let mut params = state.lock().unwrap();
    params.cm360 = cm360;
    params.dpi = dpi;
    params.normal_fov = normal_fov;
    params.scoped_fov = scoped_fov;
    println!("Mouse parameters set - cm/360: {}, DPI: {}, NormalFOV: {}, ZoomedFOV: {}", cm360, dpi, normal_fov, scoped_fov);
}

#[tauri::command]
fn get_initial_values(state: State<'_, Arc<Mutex<UserSettings>>>) -> UserSettings {
    let params = state.lock().unwrap();
    UserSettings {
        cm360: params.cm360,
        dpi: params.dpi,
        normal_fov: params.normal_fov,
        scoped_fov: params.scoped_fov,
    }
}

#[tauri::command]
fn set_current_page(page: String, state: State<'_, Arc<Mutex<AppState>>>) {
    let mut app_state = state.lock().unwrap();
    app_state.current_page = page;
    println!("Current page set to {}", app_state.current_page);
}

fn move_mouse_by(x: i32, y: i32) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_relative(x, y);
    println!("Mouse moved by ({}, {}) counts", x, y);
}

fn setup_global_shortcut(handle: AppHandle) {
    let global_shortcut_manager = handle.global_shortcut_manager();

    // Make the global_shortcut_manager mutable
    let mut global_shortcut_manager = global_shortcut_manager.clone();

    let app_handle = handle.clone();

    global_shortcut_manager.register("F1", move || {
        let app_state = APP_STATE.lock().unwrap().as_ref().unwrap().clone();
        let state: State<Arc<Mutex<UserSettings>>> = app_handle.state();
        let mut app_state = app_state.lock().unwrap();
        let params = state.lock().unwrap();

        match app_state.current_page.as_str() {
            "main_sensitivity" => {
                let counts = calculate_counts(params.cm360, params.dpi);
                move_mouse_by(counts, 0);
            },
            "scoped_sensitivity" => {
                let counts = calculate_scoped_counts(params.cm360, params.dpi, params.normal_fov, params.scoped_fov);
                move_mouse_by(counts, 0);
            },
            "measure_fov" => {
                println!("F1 pressed on measure FOV page");
                if app_state.tracker.tracking {
                    app_state.tracker.stop_tracking().unwrap();
                    println!("Tracking stopped. Counts: {}", app_state.tracker.count);
                } else {
                    let window = app_handle.get_window("main").unwrap();
                    let hwnd = match window.hwnd() {
                        Ok(hwnd) => hwnd.0 as HWND,
                        Err(_) => panic!("Failed to get window handle"),
                    };
                    app_state.tracker.start_tracking(hwnd).unwrap();
                    println!("Tracking started.");
                }
            },
            _ => {
                println!("F1 pressed on unknown page");
            }
        }
    }).unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(UserSettings { cm360: 0.0, dpi: 0, normal_fov: 0.0, scoped_fov: 0.0 })))
        .manage(Arc::new(Mutex::new(AppState {
            current_page: "main_sensitivity".to_string(),
            tracker: MouseTracker::new(),
        })))
        .setup(|app| {
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

            setup_global_shortcut(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_user_settings, set_current_page, get_initial_values])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

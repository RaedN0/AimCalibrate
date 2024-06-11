#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, MouseControllable};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, GlobalShortcutManager, Manager, State};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{SetWindowLongPtrW, GWLP_WNDPROC};

mod mouse_tracker;
use mouse_tracker::{AppState, MouseTracker, APP_STATE};

mod calculations;
use calculations::{calculate_counts, calculate_scoped_counts, calculate_yaw, estimate_fov};

#[derive(Serialize)]
struct UserSettings {
    cm360: f64,
    dpi: i32,
    normal_fov: f64,
    scoped_fov: f64,
    game_sens: f64,
    game_fov: f64,
}

#[derive(Serialize)]
struct AppSettings {
    turn_speed: f32,
}

#[derive(Clone, serde::Serialize)]
struct FovUpdatePayload {
    fov16: f64,
}

#[tauri::command]
fn set_app_settings(turn_speed: Option<f32>, state: State<'_, Arc<Mutex<AppSettings>>>) {
    let mut params = state.lock().unwrap();

    params.turn_speed = turn_speed.unwrap_or(params.turn_speed);
}

#[tauri::command]
fn set_user_settings(
    cm360: Option<f64>,
    dpi: Option<i32>,
    normal_fov: Option<f64>,
    scoped_fov: Option<f64>,
    game_sens: Option<f64>,
    game_fov: Option<f64>,
    state: State<'_, Arc<Mutex<UserSettings>>>,
) {
    let mut params = state.lock().unwrap();

    params.cm360 = cm360.unwrap_or(params.cm360);
    params.dpi = dpi.unwrap_or(params.dpi);
    params.normal_fov = normal_fov.unwrap_or(params.normal_fov);
    params.scoped_fov = scoped_fov.unwrap_or(params.scoped_fov);
    params.game_sens = game_sens.unwrap_or(params.game_sens);
    params.game_fov = game_fov.unwrap_or(params.game_fov);
}

#[tauri::command]
fn get_initial_values(state: State<'_, Arc<Mutex<UserSettings>>>) -> UserSettings {
    let params = state.lock().unwrap();
    UserSettings {
        cm360: params.cm360,
        dpi: params.dpi,
        normal_fov: params.normal_fov,
        scoped_fov: params.scoped_fov,
        game_sens: params.game_sens,
        game_fov: params.game_fov,
    }
}

#[tauri::command]
fn set_current_page(page: String, state: State<'_, Arc<Mutex<AppState>>>) {
    let mut app_state = state.lock().unwrap();
    app_state.current_page = page;
}

fn move_mouse_by(mut x: i32, steps: i32) {
    let mut enigo = Enigo::new();

    let step_count = x / steps;
    while x > 0 {
        if x > step_count {
            enigo.mouse_move_relative(step_count, 0);
        } else {
            enigo.mouse_move_relative(x, 0);
        }
        x -= step_count;
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn setup_global_shortcut(handle: AppHandle) {
    let mut global_shortcut_manager = handle.global_shortcut_manager();

    let app_handle = handle.clone();

    global_shortcut_manager
        .register("F1", move || {
            let app_state = APP_STATE.lock().unwrap().as_ref().unwrap().clone();
            let state: State<Arc<Mutex<UserSettings>>> = app_handle.state();
            let mut app_state = app_state.lock().unwrap();
            let params = state.lock().unwrap();
            let settings_state: State<Arc<Mutex<AppSettings>>> = app_handle.state();
            let settings_params = settings_state.lock().unwrap();

            match app_state.current_page.as_str() {
                "main_sensitivity" => {
                    let counts = calculate_counts(params.cm360, params.dpi);
                    move_mouse_by(counts, (50 as f32 / settings_params.turn_speed) as i32);
                }
                "scoped_sensitivity" => {
                    let counts = calculate_scoped_counts(
                        params.cm360,
                        params.dpi,
                        params.normal_fov,
                        params.scoped_fov,
                    );
                    move_mouse_by(counts, (50 as f32 / settings_params.turn_speed) as i32);
                }
                "measure_fov" => {
                    if app_state.tracker.tracking {
                        app_state.tracker.stop_tracking().unwrap();

                        let inches_per_360 = params.cm360 / 2.54;
                        let counts = inches_per_360 * params.dpi as f64;

                        let fov = estimate_fov(
                            params.game_sens,
                            calculate_yaw(counts as i32, params.game_sens),
                            app_state.tracker.count,
                        );

                        app_handle
                            .emit_all("fov_update", FovUpdatePayload { fov16: fov })
                            .unwrap();
                    } else {
                        let window = app_handle.get_window("main").unwrap();
                        let hwnd = match window.hwnd() {
                            Ok(hwnd) => hwnd.0 as HWND,
                            Err(_) => panic!("Failed to get window handle"),
                        };
                        app_state.tracker.start_tracking(hwnd).unwrap();
                    }
                }
                _ => {
                    println!("F1 pressed on unknown page");
                }
            }
        })
        .unwrap();
}

fn main() {
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
        .manage(Arc::new(Mutex::new(AppSettings { turn_speed: 1.0 })))
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
            set_app_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

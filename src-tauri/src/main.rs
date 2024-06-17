#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, MouseControllable};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, GlobalShortcutManager, Manager, State};
use winapi::shared::windef::HWND;
use winapi::um::winnt::PMCCounter;
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

#[derive(Serialize, Deserialize)]
struct AppSettings {
    turn_speed: f32,
    hotkey1: String,
    hotkey2: String,
    hotkey3: String,
    hotkey4: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            turn_speed: 1.0,
            hotkey1: "F1".to_string(),
            hotkey2: "F2".to_string(),
            hotkey3: "F3".to_string(),
            hotkey4: "F4".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct YawStuff {
    sens: f64,
    counts: i32,
    inc: f64,
    yaw: f64,
    lower_limit: f64,
    upper_limit: f64,
}

#[derive(Clone, serde::Serialize)]
struct FovUpdatePayload {
    fov16: f64,
}

#[tauri::command]
fn set_app_settings(
    turn_speed: Option<f32>,
    hotkey1: Option<String>,
    hotkey2: Option<String>,
    hotkey3: Option<String>,
    hotkey4: Option<String>,
    state: State<'_, Arc<Mutex<AppSettings>>>,
    app_handle: AppHandle,
) {
    {
        let mut params = state.lock().unwrap();

        params.turn_speed = turn_speed.unwrap_or(params.turn_speed);
        params.hotkey1 = hotkey1.unwrap_or(params.hotkey1.clone());
        params.hotkey2 = hotkey2.unwrap_or(params.hotkey2.clone());
        params.hotkey3 = hotkey3.unwrap_or(params.hotkey3.clone());
        params.hotkey4 = hotkey4.unwrap_or(params.hotkey4.clone());
    }

    save_app_settings(state.clone()).expect("Failed to save Settings");

    setup_global_shortcut(app_handle);
}

#[tauri::command]
fn get_app_settings(state: State<'_, Arc<Mutex<AppSettings>>>) -> AppSettings {
    let params = state.lock().unwrap();
    AppSettings {
        turn_speed: params.turn_speed,
        hotkey1: params.hotkey1.clone(),
        hotkey2: params.hotkey2.clone(),
        hotkey3: params.hotkey3.clone(),
        hotkey4: params.hotkey4.clone(),
    }
}

#[tauri::command]
fn set_yaw_stuff(
    sens: Option<f64>,
    counts: Option<i32>,
    inc: Option<f64>,
    yaw: Option<f64>,
    lower_limit: Option<f64>,
    upper_limit: Option<f64>,
    state: State<'_, Arc<Mutex<YawStuff>>>,
) {
    let mut params = state.lock().unwrap();

    params.sens = sens.unwrap_or(params.sens);
    params.counts = counts.unwrap_or(params.counts);
    params.inc = inc.unwrap_or(params.inc);
    params.yaw = yaw.unwrap_or(params.yaw);
    params.lower_limit = lower_limit.unwrap_or(params.lower_limit);
    params.upper_limit = upper_limit.unwrap_or(params.upper_limit);
}

#[tauri::command]
fn get_yaw_stuff(state: State<'_, Arc<Mutex<YawStuff>>>) -> YawStuff {
    let params = state.lock().unwrap();
    YawStuff {
        sens: params.sens,
        counts: params.counts,
        inc: params.inc,
        yaw: params.yaw,
        lower_limit: params.lower_limit,
        upper_limit: params.upper_limit,
    }
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
    let state: State<Arc<Mutex<AppSettings>>> = handle.state();
    let params = state.lock().unwrap();
    let hotkey1 = params.hotkey1.clone();
    let hotkey2 = params.hotkey2.clone();
    let hotkey3 = params.hotkey3.clone();
    let hotkey4 = params.hotkey4.clone();

    let mut global_shortcut_manager = handle.global_shortcut_manager();

    let app_handle = handle.clone();

    global_shortcut_manager.unregister_all().unwrap(); // Unregister any existing shortcuts

    global_shortcut_manager
        .register(&hotkey1, move || {
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
                "measure_yaw" => {
                    println!("hallo");
                }
                _ => {
                    println!("Hotkey pressed on unknown page");
                }
            }
        })
        .unwrap();

    global_shortcut_manager
        .register(&hotkey2, move || {
            println!("Hotkey2 action here");
            // Implement your custom action for the second hotkey here
        })
        .unwrap();
}

fn save_app_settings(
    state: State<Arc<Mutex<AppSettings>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let settings = state.lock().unwrap();
    let path = get_settings_path();
    let data = serde_json::to_string(&*settings)?;
    fs::write(path, data)?;
    Ok(())
}

fn load_app_settings() -> Result<AppSettings, Box<dyn std::error::Error>> {
    let path = get_settings_path();
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let mut settings: AppSettings = serde_json::from_str(&data)?;
        if settings.hotkey1 == "" || settings.hotkey1 == "Unidentified" {
            settings.hotkey1 = "F1".to_string();
        }
        if settings.hotkey2 == "" || settings.hotkey2 == "Unidentified" {
            settings.hotkey2 = "F2".to_string();
        }
        if settings.hotkey3 == "" || settings.hotkey3 == "Unidentified" {
            settings.hotkey3 = "F3".to_string();
        }
        Ok(settings)
    } else {
        let default_settings = AppSettings::default();
        let data = serde_json::to_string(&default_settings)?;
        fs::write(&path, data)?;
        Ok(default_settings)
    }
}

fn get_settings_path() -> PathBuf {
    let config_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
        .expect("Failed to get config directory")
        .join("AimCalibrate");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    config_dir.join("settings.json")
}

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
            sens: 0.0,
            counts: 0,
            inc: 0.0,
            yaw: 0.0,
            lower_limit: 0.0,
            upper_limit: 1000.0
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
            set_yaw_stuff,
            get_yaw_stuff
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

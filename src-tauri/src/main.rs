#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use enigo::{Enigo, MouseControllable};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
use calculations::{calculate_counts, calculate_scoped_counts, calculate_yaw, estimate_fov};

#[derive(Serialize, Deserialize, Clone)]
struct YawStuff {
    sens: f64,
    counts: i32,
    inc: f64,
    yaw: f64,
    lower_limit: f64,
    upper_limit: f64,
}

#[derive(Serialize, Deserialize)]
struct GameYaw {
    name: String,
    yaw: f64,
}

#[derive(Serialize, Clone)]
struct UserSettings {
    cm360: f64,
    dpi: i32,
    normal_fov: f64,
    scoped_fov: f64,
    game_sens: f64,
    game_fov: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppSettings {
    turn_speed: f32,
    hotkeys: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            turn_speed: 1.0,
            hotkeys: vec!["F1".to_string(), "F2".to_string(), "F3".to_string(), "F4".to_string()],
        }
    }
}

#[derive(Clone, serde::Serialize)]
struct FovUpdatePayload {
    fov16: f64,
}

#[tauri::command]
fn set_app_settings(
    turn_speed: Option<f32>,
    hotkeys: Option<Vec<String>>,
    state: State<'_, Arc<Mutex<AppSettings>>>,
    app_handle: AppHandle,
) {
    {
        let mut params = state.lock().unwrap();

        params.turn_speed = turn_speed.unwrap_or(params.turn_speed);
        params.hotkeys = hotkeys.unwrap_or(params.hotkeys.clone());
    }

    save_app_settings(state.clone()).expect("Failed to save Settings");

    setup_global_shortcuts(app_handle);
}

#[tauri::command]
fn get_app_settings(state: State<'_, Arc<Mutex<AppSettings>>>) -> AppSettings {
    state.lock().unwrap().clone()
}

#[tauri::command]
fn set_yaw_stuff(sens: Option<f64>, state: State<'_, Arc<Mutex<YawStuff>>>) -> YawStuff {
    let mut params = state.lock().unwrap();

    params.sens = sens.unwrap_or(params.sens);
    params.yaw = params.inc / params.sens;
    params.lower_limit = params.yaw * 0.9;
    params.upper_limit = params.yaw * 1.1;

    params.clone()
}

#[tauri::command]
fn get_yaw_stuff(state: State<'_, Arc<Mutex<YawStuff>>>) -> YawStuff {
    state.lock().unwrap().clone()
}

#[tauri::command]
fn save_game_yaw(name: String, state: State<'_, Arc<Mutex<YawStuff>>>) {
    let params = state.lock().unwrap();
    let game_yaw = GameYaw {
        name,
        yaw: params.yaw,
    };

    let path = get_yaw_file_path();
    let mut game_yaws = load_yaw_data(&path).unwrap_or_default();

    game_yaws.push(game_yaw);

    save_yaw_data(&path, &game_yaws).expect("Failed to save yaw data");
    println!("Game yaw data saved successfully.");
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
    state.lock().unwrap().clone()
}

#[tauri::command]
fn set_current_page(page: String, state: State<'_, Arc<Mutex<AppState>>>) {
    let mut app_state = state.lock().unwrap();
    app_state.current_page = page;
}

fn move_mouse_by(mut x: i32, steps: i32, right: bool) {
    #[cfg(target_os = "windows")]
    {
        let mut enigo = Enigo::new();

        let step_count = x / steps;
        while x > 0 {
            if right {
                let move_by = if x > step_count { step_count } else { x };
                enigo.mouse_move_relative(move_by, 0);
                x -= move_by;
            } else {
                let move_by = if x > step_count { step_count } else { x };
                enigo.mouse_move_relative(-move_by, 0);
                x -= move_by;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        println!("Mock mouse movement on macOS");
    }
}

fn setup_global_shortcuts(handle: AppHandle) {
    let settings_state: State<Arc<Mutex<AppSettings>>> = handle.state();
    let params = settings_state.lock().unwrap();
    let hotkeys = params.hotkeys.clone();

    let mut global_shortcut_manager = handle.global_shortcut_manager();
    global_shortcut_manager.unregister_all().unwrap();

    let app_handle = handle.clone();

    for (index, hotkey) in hotkeys.iter().enumerate() {
        let app_handle = app_handle.clone();
        global_shortcut_manager
            .register(hotkey, move || {
                handle_hotkey(index, &app_handle);
            })
            .unwrap();
    }
}

fn handle_hotkey(index: usize, app_handle: &AppHandle) {
    let app_state = APP_STATE.lock().unwrap().as_ref().unwrap().clone();
    let mut app_state = app_state.lock().unwrap();
    let params_state: State<Arc<Mutex<UserSettings>>> = app_handle.state();
    let params = params_state.lock().unwrap();
    let settings_state: State<Arc<Mutex<AppSettings>>> = app_handle.state();
    let settings_params = settings_state.lock().unwrap();
    let yaw_state: State<Arc<Mutex<YawStuff>>> = app_handle.state();
    let mut yaw_params = yaw_state.lock().unwrap();

    match index {
        0 => {
            // Hotkey 1 action
            match app_state.current_page.as_str() {
                "main_sensitivity" => {
                    let counts = calculate_counts(params.cm360, params.dpi);
                    move_mouse_by(
                        counts,
                        (50.0 / settings_params.turn_speed) as i32,
                        true,
                    );
                }
                "scoped_sensitivity" => {
                    let counts = calculate_scoped_counts(
                        params.cm360,
                        params.dpi,
                        params.normal_fov,
                        params.scoped_fov,
                    );
                    move_mouse_by(
                        counts,
                        (50.0 / settings_params.turn_speed) as i32,
                        true,
                    );
                }
                "measure_fov" => {
                    if app_state.tracker.tracking {
                        app_state.tracker.stop_tracking().unwrap();

                        let inches_per_360 = params.cm360 / 2.54;
                        let counts = inches_per_360 * params.dpi as f64;

                        let fov = estimate_fov(
                            params.game_sens,
                            calculate_yaw(counts as i32, params.game_sens),
                            app_state.tracker.count.abs(),
                        );

                        app_handle
                            .emit_all("fov_update", FovUpdatePayload { fov16: fov })
                            .unwrap();
                    } else {
                        start_tracking(app_handle, &mut app_state);
                    }
                }
                "measure_yaw" => {
                    if app_state.tracker.tracking {
                        app_state.tracker.stop_tracking().unwrap();

                        yaw_params.counts = app_state.tracker.count.abs();
                        yaw_params.inc = 360.0 / yaw_params.counts as f64;
                        yaw_params.yaw = yaw_params.inc / yaw_params.sens;
                        yaw_params.lower_limit = yaw_params.yaw * 0.9;
                        yaw_params.upper_limit = yaw_params.yaw * 1.1;

                        app_handle
                            .emit_all("yaw_update", yaw_params.clone())
                            .unwrap();
                    } else {
                        start_tracking(app_handle, &mut app_state);
                    }
                }
                _ => {
                    println!("Hotkey pressed on unknown page");
                }
            }
        }
        1 => {
            // Hotkey 2 action
            if app_state.current_page == "measure_yaw" {
                let settings_params = settings_state.lock().unwrap();
                move_mouse_by(
                    yaw_params.counts,
                    (50.0 / settings_params.turn_speed) as i32,
                    true,
                );
            }
        }
        2 => {
            // Hotkey 3 action
            if app_state.current_page == "measure_yaw" {
                let settings_params = settings_state.lock().unwrap();
                move_mouse_by(
                    yaw_params.counts,
                    (50.0 / settings_params.turn_speed) as i32,
                    false,
                );

                yaw_params.upper_limit = yaw_params.yaw;
                yaw_params.yaw = (yaw_params.upper_limit + yaw_params.lower_limit) / 2.0;
                yaw_params.inc = yaw_params.sens * yaw_params.yaw;
                yaw_params.counts = (360.0 / yaw_params.inc).round() as i32;

                app_handle
                    .emit_all("yaw_update", yaw_params.clone())
                    .unwrap();
            }
        }
        3 => {
            // Hotkey 4 action
            if app_state.current_page == "measure_yaw" {
                let settings_params = settings_state.lock().unwrap();
                move_mouse_by(
                    yaw_params.counts,
                    (50.0 / settings_params.turn_speed) as i32,
                    false,
                );

                yaw_params.lower_limit = yaw_params.yaw;
                yaw_params.yaw = (yaw_params.upper_limit + yaw_params.lower_limit) / 2.0;
                yaw_params.inc = yaw_params.sens * yaw_params.yaw;
                yaw_params.counts = (360.0 / yaw_params.inc).round() as i32;

                app_handle
                    .emit_all("yaw_update", yaw_params.clone())
                    .unwrap();
            }
        }
        _ => {
            println!("Unknown hotkey index: {}", index);
        }
    }
}

#[cfg(target_os = "windows")]
fn start_tracking(app_handle: &AppHandle, app_state: &mut AppState) {
    let window = app_handle.get_window("main").unwrap();
    let hwnd = match window.hwnd() {
        Ok(hwnd) => hwnd.0 as HWND,
        Err(_) => panic!("Failed to get window handle"),
    };
    app_state.tracker.start_tracking(hwnd).unwrap();
}

#[cfg(not(target_os = "windows"))]
fn start_tracking(_app_handle: &AppHandle, app_state: &mut AppState) {
    app_state.tracker.start_tracking().unwrap();
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
        let data = fs::read_to_string(&path)?;
        // Use serde_json's `from_str` with `default` to fill missing fields
        let mut settings: AppSettings = serde_json::from_str(&data).unwrap_or_else(|_| AppSettings::default());
        
        // Ensure that the `hotkeys` vector is populated
        if settings.hotkeys.is_empty() {
            settings.hotkeys = vec![
                "F1".to_string(),
                "F2".to_string(),
                "F3".to_string(),
                "F4".to_string(),
            ];
        }

        // Save the updated settings back to the file
        let updated_data = serde_json::to_string(&settings)?;
        fs::write(&path, updated_data)?;

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

fn get_yaw_file_path() -> PathBuf {
    let config_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
        .expect("Failed to get config directory")
        .join("AimCalibrate");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    config_dir.join("Games.json")
}

fn load_yaw_data(path: &PathBuf) -> Result<Vec<GameYaw>, Box<dyn std::error::Error>> {
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let game_yaws: Vec<GameYaw> = serde_json::from_str(&data)?;
        Ok(game_yaws)
    } else {
        Ok(vec![])
    }
}

fn save_yaw_data(
    path: &PathBuf,
    game_yaws: &Vec<GameYaw>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(game_yaws)?;
    fs::write(path, data)?;
    Ok(())
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
            set_yaw_stuff,
            get_yaw_stuff,
            save_game_yaw
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use crate::calculations::{calculate_counts, calculate_scoped_counts, calculate_yaw, estimate_fov};
use crate::models::{AppSettings, CmUpdatePayload, FovUpdatePayload, GameYaw, UserSettings, YawStuff};
#[cfg(target_os = "windows")]
use crate::mouse_tracker::{AppState, APP_STATE};
#[cfg(not(target_os = "windows"))]
use crate::mouse_tracker_mock::{AppState, APP_STATE};
use enigo::{Enigo, Mouse, Settings};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, GlobalShortcutManager, Manager, State};
#[cfg(target_os = "windows")]
use winapi::shared::windef::HWND;

pub fn move_mouse_by(mut x: i32, steps: i32, right: bool) {
    let settings = Settings {
        windows_subject_to_mouse_speed_and_acceleration_level: true,
        ..Default::default()
    };

    let mut enigo = Enigo::new(&settings).unwrap();

    let step_count = x / steps;
    while x > 0 {
        if right {
            let move_by = if x > step_count { step_count } else { x };
            enigo.move_mouse(move_by, 0, enigo::Coordinate::Rel).unwrap();
            x -= move_by;
        } else {
            let move_by = if x > step_count { step_count } else { x };
            enigo.move_mouse(-move_by, 0, enigo::Coordinate::Rel).unwrap();
            x -= move_by;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

pub fn setup_global_shortcuts(handle: AppHandle) {
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
                crate::utils::handle_hotkey(index, &app_handle);
            })
            .unwrap();
    }
}

pub fn handle_hotkey(index: usize, app_handle: &AppHandle) {
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
            match app_state.current_page.as_str() {
                "main_sensitivity" => {
                    if app_state.tracker.tracking {
                        app_state.tracker.stop_tracking().unwrap();

                        let cm_per360 = 2.54 * app_state.tracker.count.abs() as f64 / params.dpi as f64;

                        app_handle
                            .emit_all("cm_update", CmUpdatePayload { cm_per360 })
                            .unwrap();
                    } else {
                        start_tracking(app_handle, &mut app_state);
                    }
                }
                "measure_yaw" => {
                    move_mouse_by(
                        yaw_params.counts,
                        (50.0 / settings_params.turn_speed) as i32,
                        true,
                    );
                }
                _ => {
                    println!("Hotkey pressed on unknown page");
                }
            }
        }
        2 => {
            // Hotkey 3 action
            if app_state.current_page == "measure_yaw" {
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
pub fn start_tracking(app_handle: &AppHandle, app_state: &mut AppState) {
    let window = app_handle.get_window("main").unwrap();
    let hwnd = match window.hwnd() {
        Ok(hwnd) => hwnd.0 as HWND,
        Err(_) => panic!("Failed to get window handle"),
    };
    app_state.tracker.start_tracking(hwnd).unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn start_tracking(_app_handle: &AppHandle, app_state: &mut AppState) {
    app_state.tracker.start_tracking().unwrap();
}

pub fn save_app_settings(
    state: State<Arc<Mutex<AppSettings>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let settings = state.lock().unwrap();
    let path = get_settings_path();
    let data = serde_json::to_string(&*settings)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn load_app_settings() -> Result<AppSettings, Box<dyn std::error::Error>> {
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

pub fn get_settings_path() -> PathBuf {
    let config_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
        .expect("Failed to get config directory")
        .join("AimCalibrate");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    config_dir.join("settings.json")
}

pub fn get_yaw_file_path() -> PathBuf {
    let config_dir = tauri::api::path::app_config_dir(&tauri::Config::default())
        .expect("Failed to get config directory")
        .join("AimCalibrate");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    config_dir.join("Games.json")
}

pub fn load_yaw_data(path: &PathBuf) -> Result<Vec<GameYaw>, Box<dyn std::error::Error>> {
    if path.exists() {
        let data = fs::read_to_string(path)?;
        let game_yaws: Vec<GameYaw> = serde_json::from_str(&data)?;
        Ok(game_yaws)
    } else {
        Ok(vec![])
    }
}

pub fn save_yaw_data(
    path: &PathBuf,
    game_yaws: &Vec<GameYaw>,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(game_yaws)?;
    fs::write(path, data)?;
    Ok(())
}
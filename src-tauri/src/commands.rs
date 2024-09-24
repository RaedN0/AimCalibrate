use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};
use crate::models::{AppSettings, GameYaw, UserSettings, YawStuff};
use crate::mouse_tracker_mock::AppState;
use crate::utils::{get_yaw_file_path, load_yaw_data, save_app_settings, save_yaw_data, setup_global_shortcuts};

#[tauri::command]
pub fn set_app_settings(
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
pub fn get_app_settings(state: State<'_, Arc<Mutex<AppSettings>>>) -> AppSettings {
    state.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_yaw_values(sens: Option<f64>, state: State<'_, Arc<Mutex<YawStuff>>>) -> YawStuff {
    let mut params = state.lock().unwrap();

    params.sens = sens.unwrap_or(params.sens);
    params.yaw = params.inc / params.sens;
    params.lower_limit = params.yaw * 0.9;
    params.upper_limit = params.yaw * 1.1;

    params.clone()
}

#[tauri::command]
pub fn get_yaw_values(state: State<'_, Arc<Mutex<YawStuff>>>) -> YawStuff {
    state.lock().unwrap().clone()
}

#[tauri::command]
pub fn save_game_yaw(name: String, state: State<'_, Arc<Mutex<YawStuff>>>) {
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
pub fn set_user_settings(
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
pub fn get_initial_values(state: State<'_, Arc<Mutex<UserSettings>>>) -> UserSettings {
    state.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_current_page(page: String, state: State<'_, Arc<Mutex<AppState>>>) {
    let mut app_state = state.lock().unwrap();
    app_state.current_page = page;
}

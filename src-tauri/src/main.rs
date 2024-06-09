use std::{sync::{Arc, Mutex}};
use tauri::{GlobalShortcutManager, Manager, State, AppHandle};
use enigo::{Enigo, MouseControllable};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{GetWindowLongPtrW, SetWindowLongPtrW, GWLP_USERDATA, GWLP_WNDPROC};

mod mouse_tracker;
use mouse_tracker::{MouseTracker, window_proc};

struct MouseParameters {
    cm360: f32,
    dpi: i32,
}

struct AppState {
    current_page: String,
}

#[tauri::command]
fn set_mouse_parameters(cm360: f32, dpi: i32, state: State<'_, Arc<Mutex<MouseParameters>>>) {
    let mut params = state.lock().unwrap();
    params.cm360 = cm360;
    params.dpi = dpi;
    println!("Mouse parameters set - cm/360: {}, DPI: {}", cm360, dpi);
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

fn calculate_counts(cm: f32, dpi: i32) -> i32 {
    let inches_per360 = cm / 2.54;
    let counts_per360 = inches_per360 * dpi as f32;
    counts_per360 as i32
}

fn setup_global_shortcut(handle: AppHandle) {
    let global_shortcut_manager = handle.global_shortcut_manager();

    // Make the global_shortcut_manager mutable
    let mut global_shortcut_manager = global_shortcut_manager.clone();

    let app_handle = handle.clone();

    global_shortcut_manager.register("F1", move || {
        let app_state: State<Arc<Mutex<AppState>>> = app_handle.state();
        let state: State<Arc<Mutex<MouseParameters>>> = app_handle.state();
        let current_page = app_state.lock().unwrap().current_page.clone();
        let params = state.lock().unwrap();

        match current_page.as_str() {
            "main_sensitivity" => {
                let counts = calculate_counts(params.cm360, params.dpi);
                move_mouse_by(counts, 0);
            },
            "scoped_sensitivity" => {
                println!("F1 pressed on scoped sensitivity page");
            },
            "measure_fov" => {
                println!("F1 pressed on measure FOV page");
            },
            _ => {
                println!("F1 pressed on unknown page");
            }
        }
    }).unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(MouseParameters { cm360: 0.0, dpi: 0 })))
        .manage(Arc::new(Mutex::new(AppState { current_page: "main_sensitivity".to_string() })))
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let hwnd = match window.hwnd() {
                Ok(hwnd) => hwnd.0 as HWND,
                Err(_) => panic!("Failed to get window handle"),
            };

            let mut tracker = MouseTracker::new();
            if !tracker.start_tracking(hwnd) {
                println!("Failed to register raw input device.");
            }

            unsafe {
                SetWindowLongPtrW(hwnd, GWLP_USERDATA, &tracker as *const _ as isize);
                SetWindowLongPtrW(hwnd, GWLP_WNDPROC, window_proc as isize);
            }

            setup_global_shortcut(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_mouse_parameters, set_current_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

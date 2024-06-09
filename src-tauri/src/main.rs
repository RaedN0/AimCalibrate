use std::sync::{Arc, Mutex};
use tauri::{GlobalShortcutManager, Manager, State, AppHandle};
use enigo::{Enigo, MouseControllable};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{SetWindowLongPtrW, GWLP_WNDPROC, DefWindowProcW, WM_INPUT};
use winapi::shared::minwindef::{LRESULT, UINT, WPARAM, LPARAM};
use std::ptr::null_mut;
use std::ptr::NonNull;
use lazy_static::lazy_static;

mod mouse_tracker;
use mouse_tracker::MouseTracker;

struct MouseParameters {
    cm360: f32,
    dpi: i32,
}

struct AppState {
    current_page: String,
    tracker: MouseTracker,
}

lazy_static! {
    static ref APP_STATE: Mutex<Option<Arc<Mutex<AppState>>>> = Mutex::new(None);
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
        let app_state = APP_STATE.lock().unwrap().as_ref().unwrap().clone();
        let state: State<Arc<Mutex<MouseParameters>>> = app_handle.state();
        let mut app_state = app_state.lock().unwrap();
        let params = state.lock().unwrap();

        match app_state.current_page.as_str() {
            "main_sensitivity" => {
                let counts = calculate_counts(params.cm360, params.dpi);
                move_mouse_by(counts, 0);
            },
            "scoped_sensitivity" => {
                println!("F1 pressed on scoped sensitivity page");
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

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_INPUT => {
            let app_state = APP_STATE.lock().unwrap().as_ref().unwrap().clone();
            let mut tracker = app_state.lock().unwrap();

            let raw_input = std::ptr::NonNull::new(lparam as *mut std::ffi::c_void).unwrap();
            match tracker.tracker.get_raw_input_data(raw_input) {
                Ok(raw_input_data) => {
                    let x_movement = raw_input_data.data.mouse().lLastX;
                    tracker.tracker.count += x_movement;
                }
                Err(e) => eprintln!("Failed to get raw input data: {}", e),
            }
        }
        _ => {}
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn main() {
    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(MouseParameters { cm360: 0.0, dpi: 0 })))
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
                SetWindowLongPtrW(hwnd, GWLP_WNDPROC, window_proc as isize);
            }

            setup_global_shortcut(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_mouse_parameters, set_current_page])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

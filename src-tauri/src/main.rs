// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::{Arc, Mutex}};
use tauri::{GlobalShortcutManager, Manager, State};
use enigo::{Enigo, MouseControllable};

struct MouseParameters {
    cm360: f32,
    dpi: i32,
}

#[tauri::command]
fn set_mouse_parameters(cm360: f32, dpi: i32, state: State<'_, Arc<Mutex<MouseParameters>>>) {
    let mut params = state.lock().unwrap();
    params.cm360 = cm360;
    params.dpi = dpi;
    println!("Mouse parameters set - cm/360: {}, DPI: {}", cm360, dpi);
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

fn main() {
    tauri::Builder::default()
    .manage(Arc::new(Mutex::new(MouseParameters { cm360: 0.0, dpi: 0 })))
        .setup(|app| {
            let handle = app.handle();
            let global_shortcut_manager = handle.global_shortcut_manager();

            // Make the global_shortcut_manager mutable
            let mut global_shortcut_manager = global_shortcut_manager;

            global_shortcut_manager.register("F1", move || {
                let state: State<Arc<Mutex<MouseParameters>>> = handle.state();
                let params = state.lock().unwrap();
                let counts = calculate_counts(params.cm360, params.dpi);
                move_mouse_by(counts, 0)
            }).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set_mouse_parameters])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
    
use std::ptr::null_mut;
use std::sync::{Arc, Mutex};
use winapi::shared::minwindef::{LRESULT, UINT, WPARAM, LPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{DefWindowProcW, RegisterRawInputDevices, RAWINPUTDEVICE, RIDEV_INPUTSINK, RIDEV_REMOVE, WM_INPUT, GetRawInputData, RID_INPUT, RAWINPUT, RAWINPUTHEADER};
use std::ptr::NonNull;

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

    pub fn start_tracking(&mut self, handle: HWND) -> Result<(), String> {
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x02,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: handle,
        };

        let devices = [rid];
        let result = unsafe {
            RegisterRawInputDevices(
                devices.as_ptr(),
                devices.len() as UINT,
                std::mem::size_of::<RAWINPUTDEVICE>() as UINT,
            )
        };

        if result != 0 {
            self.tracking = true;
            self.count = 0; // Reset the count when tracking starts
            Ok(())
        } else {
            Err("Failed to register raw input device(s).".to_string())
        }
    }

    pub fn stop_tracking(&mut self) -> Result<(), String> {
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x02,
            dwFlags: RIDEV_REMOVE,
            hwndTarget: null_mut(),
        };

        let devices = [rid];
        let result = unsafe {
            RegisterRawInputDevices(
                devices.as_ptr(),
                devices.len() as UINT,
                std::mem::size_of::<RAWINPUTDEVICE>() as UINT,
            )
        };

        if result != 0 {
            self.tracking = false;
            Ok(())
        } else {
            Err("Failed to unregister raw input device(s).".to_string())
        }
    }

    pub fn update_counts(&mut self, x: i32) {
        self.count += x;
    }

    pub unsafe extern "system" fn window_proc(
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
                        tracker.tracker.update_counts(x_movement);
                    }
                    Err(e) => eprintln!("Failed to get raw input data: {}", e),
                }
            }
            _ => {}
        }

        DefWindowProcW(hwnd, msg, wparam, lparam)
    }

    pub fn get_raw_input_data(&self, h_raw_input: NonNull<std::ffi::c_void>) -> Result<RAWINPUT, String> {
        let mut size: UINT = 0;
        let header_size = std::mem::size_of::<RAWINPUTHEADER>() as UINT;

        unsafe {
            if GetRawInputData(
                h_raw_input.as_ptr() as _,
                RID_INPUT,
                null_mut(),
                &mut size,
                header_size,
            ) == 0
            {
                let mut raw_input = vec![0u8; size as usize];
                if GetRawInputData(
                    h_raw_input.as_ptr() as _,
                    RID_INPUT,
                    raw_input.as_mut_ptr() as _,
                    &mut size,
                    header_size,
                ) == size
                {
                    Ok(*(raw_input.as_ptr() as *const RAWINPUT))
                } else {
                    Err("Failed to get raw input data.".to_string())
                }
            } else {
                Err("Failed to get raw input data size.".to_string())
            }
        }
    }
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_STATE: Mutex<Option<Arc<Mutex<AppState>>>> = Mutex::new(None);
}

pub struct AppState {
    pub current_page: String,
    pub tracker: MouseTracker,
}

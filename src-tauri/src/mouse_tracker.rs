use std::mem;
use std::ptr::null_mut;
use std::ptr::NonNull;
use winapi::um::winuser::{
    RegisterRawInputDevices, GetRawInputData, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER,
    RAWMOUSE, RIDEV_INPUTSINK, RIDEV_REMOVE, RID_INPUT,
};
use winapi::shared::windef::HWND;

pub struct MouseTracker {
    pub count: i32,
    pub tracking: bool,
}

impl MouseTracker {
    pub fn new() -> Self {
        MouseTracker {
            count: 0,
            tracking: false,
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
                devices.len() as u32,
                mem::size_of::<RAWINPUTDEVICE>() as u32,
            )
        };

        if result != 0 {
            self.tracking = true;
            self.count = 0;
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
                devices.len() as u32,
                mem::size_of::<RAWINPUTDEVICE>() as u32,
            )
        };

        if result != 0 {
            self.tracking = false;
            Ok(())
        } else {
            Err("Failed to unregister raw input device(s).".to_string())
        }
    }

    pub fn get_raw_input_data(
        &self,
        h_raw_input: NonNull<std::ffi::c_void>,
    ) -> Result<RAWINPUT, String> {
        let mut size: u32 = 0;
        let header_size = mem::size_of::<RAWINPUTHEADER>() as u32;

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

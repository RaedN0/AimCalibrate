use std::ptr::null_mut;
use std::mem::size_of;
use winapi::shared::minwindef::{DWORD, LPVOID, PUINT, UINT, WPARAM, LPARAM, LRESULT, BOOL, USHORT, ULONG};
use winapi::shared::ntdef::{HANDLE, LONG};  // Import LONG directly from ntdef
use winapi::shared::windef::HWND;
use winapi::um::winuser::*;
use winapi::um::libloaderapi::GetModuleHandleW;

const RIDEV_INPUTSINK: DWORD = 0x00000100;
const RIM_TYPEMOUSE: DWORD = 0;

#[repr(C)]
struct RAWINPUTDEVICE {
    usUsagePage: USHORT,
    usUsage: USHORT,
    dwFlags: DWORD,
    hwndTarget: HWND,
}

#[repr(C)]
struct RAWINPUTHEADER {
    dwType: DWORD,
    dwSize: DWORD,
    hDevice: HANDLE,
    wParam: WPARAM,
}

#[repr(C)]
struct RAWMOUSE {
    usFlags: USHORT,
    ulButtons: ULONG,
    usButtonFlags: USHORT,
    usButtonData: USHORT,
    ulRawButtons: ULONG,
    lLastX: LONG,
    lLastY: LONG,
    ulExtraInformation: ULONG,
}

#[repr(C)]
struct RAWINPUT {
    header: RAWINPUTHEADER,
    mouse: RAWMOUSE,
}

extern "system" {
    fn RegisterRawInputDevices(pRawInputDevices: *const RAWINPUTDEVICE, uiNumDevices: UINT, cbSize: UINT) -> BOOL;
    fn GetRawInputData(hRawInput: HRAWINPUT, uiCommand: UINT, pData: LPVOID, pcbSize: PUINT, cbSizeHeader: UINT) -> UINT;
}

pub struct MouseTracker {
    tracking: bool,
}

impl MouseTracker {
    pub fn new() -> Self {
        MouseTracker { tracking: false }
    }

    pub fn start_tracking(&mut self, hwnd: HWND) -> bool {
        let rid = RAWINPUTDEVICE {
            usUsagePage: 0x01,
            usUsage: 0x02,
            dwFlags: RIDEV_INPUTSINK,
            hwndTarget: hwnd,
        };

        let result = unsafe {
            RegisterRawInputDevices(&rid as *const RAWINPUTDEVICE, 1, size_of::<RAWINPUTDEVICE>() as u32)
        };

        self.tracking = result != 0;
        self.tracking
    }

    pub fn handle_raw_input(&self, lparam: LPARAM) {
        let mut dw_size: UINT = 0;

        unsafe {
            GetRawInputData(
                lparam as HRAWINPUT,
                RID_INPUT,
                null_mut(),
                &mut dw_size as *mut UINT,
                size_of::<RAWINPUTHEADER>() as UINT,
            );

            if dw_size > 0 {
                let mut raw: Vec<u8> = Vec::with_capacity(dw_size as usize);
                GetRawInputData(
                    lparam as HRAWINPUT,
                    RID_INPUT,
                    raw.as_mut_ptr() as LPVOID,
                    &mut dw_size as *mut UINT,
                    size_of::<RAWINPUTHEADER>() as UINT,
                );

                let raw_input = &*(raw.as_ptr() as *const RAWINPUT);
                if raw_input.header.dwType == RIM_TYPEMOUSE {
                    let mouse = &raw_input.mouse;
                    println!("Mouse moved: ({}, {}) counts", mouse.lLastX, mouse.lLastY);
                }
            }
        }
    }
}

pub unsafe extern "system" fn window_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if msg == WM_INPUT {
        let mouse_tracker: &MouseTracker = &*(GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const MouseTracker);
        mouse_tracker.handle_raw_input(lparam);
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

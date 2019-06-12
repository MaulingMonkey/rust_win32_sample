// Vaguely adopted from https://docs.microsoft.com/en-us/windows/desktop/learnwin32/your-first-windows-program

#![windows_subsystem = "windows"]
#![allow(non_snake_case)] // WinAPI style

mod win32 {
    pub use winapi::shared::minwindef::*;
    pub use winapi::shared::windef::*;
    pub use winapi::um::debugapi::*;
    pub use winapi::um::libloaderapi::*;
    pub use winapi::um::wingdi::*;
    pub use winapi::um::winuser::*;
}

use win32::*;
use std::{ptr, mem};

macro_rules! wstr {
    ($str:expr) => {
        std::os::windows::ffi::OsStrExt::encode_wide(
            std::ffi::OsStr::new(concat!($str, "\0"))
        ).collect::<Vec<u16>>().as_ptr()
        // A proc_macro like wch_c (wchar 0.2.0) would let us avoid ALLOCATING for a freakin' PCWSTR literal.
        // However, that's not yet part of stable rust, so herp de derpity derp.
    };
}

macro_rules! expect {
    ($expr:expr) => {{
        if !($expr) {
            if IsDebuggerPresent() != 0 { DebugBreak(); }
            panic!(concat!("expect!(", stringify!($expr), ") failed"));
        }
    }};
}

extern "system"
fn window_proc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        match uMsg {
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            },
            WM_PAINT => {
                let mut ps : PAINTSTRUCT = mem::zeroed();
                let hdc = BeginPaint(hwnd, &mut ps);
                let brush = CreateSolidBrush(RGB(0x33, 0x66, 0x99));
                //let brush = COLOR_WINDOW as HBRUSH;
                FillRect(hdc, &ps.rcPaint, brush);
                EndPaint(hwnd, &ps);
                DeleteObject(brush as HGDIOBJ);
                0

            },
            _ => {
                DefWindowProcW(hwnd, uMsg, wParam, lParam)
            },
        }
    }
}

fn main() {
    unsafe {
        std::panic::set_hook(Box::new(|_| if IsDebuggerPresent() != 0 { DebugBreak(); } ));

        let hInstance = GetModuleHandleW(ptr::null());
        expect!(hInstance != ptr::null_mut());

        let hCursor = LoadCursorW(ptr::null_mut(), IDC_ARROW);
        expect!(hCursor != ptr::null_mut());

        let wc = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance,
            hCursor,
            lpszClassName: wstr!("SampleWndClass"),
            ..mem::zeroed()
        };
        expect!(RegisterClassW(&wc) != 0);

        let hwnd = CreateWindowExW(
            0, // window style
            wstr!("SampleWndClass"),
            wstr!("Title"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT, // x
            CW_USEDEFAULT, // y
            CW_USEDEFAULT, // nwidth
            CW_USEDEFAULT, // nheight
            ptr::null_mut(), // parent
            ptr::null_mut(), // menu
            hInstance,
            ptr::null_mut() // lpParam
        );
        expect!(hwnd != ptr::null_mut());

        let nCmdShow = SW_SHOW; // ?
        expect!(ShowWindow(hwnd, nCmdShow) == 0);

        let mut msg : MSG = mem::zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    };
}

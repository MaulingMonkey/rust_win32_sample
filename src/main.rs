// Vaguely adopted from https://docs.microsoft.com/en-us/windows/desktop/learnwin32/your-first-windows-program
// ...and https://code.msdn.microsoft.com/windowsdesktop/Direct3D-Tutorial-Win32-829979ef
//     1) https://docs.microsoft.com/en-us/previous-versions//ff729718(v=vs.85)

#![windows_subsystem = "windows"]
#![allow(non_snake_case)] // WinAPI style

mod win32 {
    pub use winapi::*;
    pub use winapi::shared::dxgi::*;
    pub use winapi::shared::dxgiformat::*;
    pub use winapi::shared::dxgitype::*;
    pub use winapi::shared::minwindef::*;
    pub use winapi::shared::windef::*;
    pub use winapi::shared::winerror::*;
    pub use winapi::um::d3d11::*;
    pub use winapi::um::d3dcommon::*;
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
            800, // nwidth
            600, // nheight
            ptr::null_mut(), // parent
            ptr::null_mut(), // menu
            hInstance,
            ptr::null_mut() // lpParam
        );
        expect!(hwnd != ptr::null_mut());

        let nCmdShow = SW_SHOW;
        expect!(ShowWindow(hwnd, nCmdShow) == 0);

        let mut rect : RECT = mem::zeroed();
        expect!(GetClientRect(hwnd, &mut rect) != 0);

        let w = (rect.right - rect.left) as u32;
        let h = (rect.bottom - rect.top) as u32;

        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width:  w,
                Height: h,
                RefreshRate: DXGI_RATIONAL { Numerator: 60, Denominator: 1 },
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_CENTERED,
            },
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            OutputWindow: hwnd,
            Windowed: 1,
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: 0,
        };

        let mut swap_chain = ptr::null_mut();
        let mut device = ptr::null_mut();
        let mut device_context = ptr::null_mut();
        let feature_level = D3D_FEATURE_LEVEL_10_0;
        let feature_levels = &[feature_level];
        expect!(SUCCEEDED(D3D11CreateDeviceAndSwapChain(
            ptr::null_mut(), // adapter
            D3D_DRIVER_TYPE_HARDWARE,
            ptr::null_mut(), // software
            0, // flags
            feature_levels.as_ptr(),
            feature_levels.len() as u32,
            D3D11_SDK_VERSION, // SDK Version
            &swap_chain_desc,
            &mut swap_chain,
            &mut device,
            ptr::null_mut(), //&mut feature_level,
            &mut device_context
        )));
        expect!(swap_chain     != ptr::null_mut());
        expect!(device         != ptr::null_mut());
        expect!(device_context != ptr::null_mut());

        let mut back_buffer = ptr::null_mut();
        expect!(SUCCEEDED((*swap_chain).GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut back_buffer)));
        let back_buffer = back_buffer as * mut winapi::um::d3d11::ID3D11Resource;

        let mut rtv = ptr::null_mut();
        expect!(SUCCEEDED((*device).CreateRenderTargetView(back_buffer, ptr::null_mut(), &mut rtv)));
        expect!(rtv != ptr::null_mut());

        (*device_context).OMSetRenderTargets(1, [rtv].as_ptr(), ptr::null_mut());

        let vp = D3D11_VIEWPORT { Width: w as f32, Height: h as f32, MinDepth: 0.0, MaxDepth: 1.0, TopLeftX: 0.0, TopLeftY: 0.0 };
        (*device_context).RSSetViewports(1, [vp].as_ptr());

        loop {
            let mut msg : MSG = mem::zeroed();
            while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT { return; }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            (*device_context).ClearRenderTargetView(rtv, &[0.1, 0.2, 0.3, 1.0]);
            (*swap_chain).Present(0, 0);
        }
    };
}

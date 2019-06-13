use crate::win32::*;
use std::cell::Cell;
use std::mem;
use std::ptr::{null, null_mut};
use std::sync::atomic::{AtomicUsize, Ordering};



const RUST_WIN32_SAMPLE_CELL : *const i8 = "RUST_WIN32_SAMPLE_CELL\0".as_ptr() as *const _;

const WRS_NOT_REGISTERED : usize = 0;
const WRS_REGISTERING : usize = 1;
const WRS_REGISTERED : usize = 2;
static WNDCLASS_REGISTRATION_STATE : AtomicUsize = AtomicUsize::new(WRS_NOT_REGISTERED);



#[repr(transparent)]
pub struct Rect(RECT);

impl Rect {
    pub fn width(&self) -> UINT { (self.0.right - self.0.left) as UINT }
    pub fn height(&self) -> UINT { (self.0.bottom - self.0.top) as UINT }
}

impl Default for Rect {
    fn default() -> Self { Self(RECT{ left: 0, right: 0, top: 0, bottom: 0 }) }
}



type WindowInternal = Cell<HWND>;

pub struct Window {
    hwnd: Box<WindowInternal>,
}

impl Drop for Window {
    fn drop(&mut self) {
        let hwnd = self.hwnd.get();
        if hwnd != null_mut() {
            unsafe {
                RemovePropA(hwnd, RUST_WIN32_SAMPLE_CELL);
                expect_ne!(FALSE, DestroyWindow(hwnd));
            }
        }
    }
}

impl Window {
    pub fn new() -> Self {
        unsafe {
            let hInstance = GetModuleHandleW(null());
            expect_ne!(null_mut(), hInstance);

            let hCursor = LoadCursorW(null_mut(), IDC_ARROW);
            expect_ne!(null_mut(), hCursor);

            if WNDCLASS_REGISTRATION_STATE.compare_exchange(WRS_NOT_REGISTERED, WRS_REGISTERING, Ordering::Acquire, Ordering::Relaxed) == Ok(WRS_NOT_REGISTERED) {
                let wc = WNDCLASSW {
                    lpfnWndProc: Some(window_proc),
                    hInstance,
                    hCursor,
                    lpszClassName: wstr!("SampleWndClass"),
                    ..mem::zeroed()
                };
                expect_ne!(RegisterClassW(&wc), 0);
                WNDCLASS_REGISTRATION_STATE.store(WRS_REGISTERED, Ordering::Relaxed);
            }
            while WNDCLASS_REGISTRATION_STATE.load(Ordering::Acquire) < WRS_REGISTERED {}

            let hwnd = Box::new(WindowInternal::new(null_mut()));
            hwnd.set(CreateWindowExW(
                0, // window style
                wstr!("SampleWndClass"),
                wstr!("Title"),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT, // x
                CW_USEDEFAULT, // y
                800, // nwidth
                600, // nheight
                null_mut(), // parent
                null_mut(), // menu
                hInstance,
                null_mut(), // lpParam
            ));
            let internal : &WindowInternal = &*hwnd;
            let ptr = internal as *const _ as *mut _;
            expect_ne!(FALSE, SetPropA(hwnd.get(), RUST_WIN32_SAMPLE_CELL, ptr));
            expect_ne!(null_mut(), hwnd.get());

            Window { hwnd }
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hwnd.get() != null_mut()
    }

    pub fn hwnd(&self) -> HWND {
        let hwnd = self.hwnd.get();
        expect_ne!(null_mut(), hwnd);
        hwnd
    }

    pub fn show(&mut self) {
        expect_eq!(FALSE, unsafe { ShowWindow(self.hwnd(), SW_SHOW) });
    }

    pub fn client_area(&self) -> Rect {
        let mut rect : Rect = Default::default();
        expect_ne!(FALSE, unsafe { GetClientRect(self.hwnd(), &mut rect.0) });
        rect
    }
}



extern "system"
fn window_proc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    unsafe {
        match uMsg {
            WM_DESTROY => {
                PostQuitMessage(0);
                0
            },
            WM_NCDESTROY => {
                let prop = GetPropA(hwnd, RUST_WIN32_SAMPLE_CELL) as *const WindowInternal;
                if prop != null() {
                    let internal : &WindowInternal = &*prop;
                    internal.set(null_mut());
                }
                DefWindowProcW(hwnd, uMsg, wParam, lParam)
            },
            _ => {
                DefWindowProcW(hwnd, uMsg, wParam, lParam)
            },
        }
    }
}

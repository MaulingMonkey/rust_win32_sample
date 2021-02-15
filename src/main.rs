// Vaguely adopted from https://docs.microsoft.com/en-us/windows/desktop/learnwin32/your-first-windows-program
// ...and https://code.msdn.microsoft.com/windowsdesktop/Direct3D-Tutorial-Win32-829979ef
//     1) https://docs.microsoft.com/en-us/previous-versions//ff729718(v=vs.85)
//     2) https://docs.microsoft.com/en-us/previous-versions//ff729719(v=vs.85)
//     3) https://docs.microsoft.com/en-us/previous-versions//ff729720(v=vs.85)

#![windows_subsystem = "windows"]
// Changes the default windows_subsystem from "console".
//
// If the program is launched from a cmd.exe window:
//
//      "console" programs will borrow console input/output, with cmd.exe not showing a new command prompt until the
//      program either terminates, or explicitly detatches via `FreeConsole()`
//
//      "windows" programs will detatch from the console - stdout will be ignored, reading stdin will fail, and cmd.exe
//      will be able to immediately display a new C:\> prompt while the program is still executing.  They can reattach
//      to the parrent console via `AttachConsole(ARRACH_PARENT_PROCESS)`, but this can be confusing as other things are
//      likely happening in said console already.
//
// If the program is launched from an explorer.exe window:
//
//      "console" programs will spawn a new console window
//      "windows" programs won't have a console at all.  They can spawn new consoles with `AllocConsole()` however.

#![allow(non_snake_case)]
// Rust will typically warn if you use variable names like `hCursor`, preferring `h_cursor` instead.
// However, several Win32 structs contain names like `hCursor`, and matching those names allows the use of:
//
//      WNDCLASSW { hCursor, ... }
//
// Instead of:
//
//      WNDCLASSW { hCursor: h_cursor, ... }
//
// Which is much less convenient ;)

use wchar::wch_c;

use winapi::Interface; // uuidof
use winapi::shared::dxgi::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::winerror::*;
use winapi::um::d3d11::*;
use winapi::um::d3dcommon::*;
use winapi::um::debugapi::*;
use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;

use std::marker::PhantomData;
use std::mem::{size_of, size_of_val, zeroed};
use std::ptr::{null, null_mut};



/// Like `assert!(...)`, but more awesome
macro_rules! expect {
    ($expr:expr) => {{
        if !($expr) {

            OutputDebugStringA(concat!(stringify!($expr), "\n... was false\n\0").as_ptr().cast());
            //
            // Because we're using `#![windows_subsystem = "windows"]`, we don't have any stderr to write to.
            // We could create a console, but there's a special debug output stream we can use that OutputDebugStringA
            // writes to for us.  This debug output stream can be viewed by:
            //
            //  1.  The "Debug Console" tab in Visual Studio Code, when attached to a program
            //  2.  The "Output" tab in Visual Studio, when attached to a program
            //  3.  The DebugView utility (https://docs.microsoft.com/en-us/sysinternals/downloads/debugview)
            //  4.  Any other debugger that processes `OUTPUT_DEBUG_STRING_EVENT`s
            //
            // This is an ideal channel for developer-only debug spam that regular users should not be exposed to.
            // Because this API expects C-style strings, it's necessary for the buffer to be `\0`-terminated!
            //
            // MSDN:        https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringa
            // Alternative: https://docs.rs/bugsalot/0.2.1/bugsalot/macro.debugln.html


            if IsDebuggerPresent() != 0 { DebugBreak(); }
            //
            // This is a hardcoded breakpoint.  This is somewhat unnecessary - you could simply remember to breakpoint
            // `rust_panic` inside your debugger.  However, hardcoding a breakpoint here means we don't have to remember
            // to configure said breakpoint.  Also, IDEs will usually navigate directly to the source code of the
            // breakpoint - `rust_panic` isn't what you want to look at, but whatever is calling this macro likely is.
            //
            // By placing this breakpoint inside a macro, which gets directly inlined into the code, 9 times out of 10
            // we'll be looking at the actual code that broke, without any manual work by the developer.  Neat!
            //
            // MSDN:        https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-isdebuggerpresent
            // MSDN:        https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-debugbreak
            // Alternative: https://docs.rs/bugsalot/0.2.1/bugsalot/debugger/fn.break_if_attached.html


            panic!(concat!("expect!(", stringify!($expr), ") failed"));
            // If no debugger is attached... well, fall back on doing what panic did anyways.

        }
    }};
}

fn main() {
    unsafe {

        std::panic::set_hook(Box::new(|_| if IsDebuggerPresent() != 0 { DebugBreak(); } ));
        // If we panic "regularly" and not via `expect!`, I still want a breakpoint - see `expect!` above for details.


        let hExeInstance = GetModuleHandleW(null());
        expect!(!hExeInstance.is_null());
        //
        // By passing `NULL` to `GetModuleHandleW`, we retrieve an `HMODULE` to the currently executing process / .exe.
        // A lot of Win32 functions treat `HMODULE`s as a namespace or container of resources.
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew


        let hCursor = LoadCursorW(null_mut(), IDC_ARROW);
        expect!(!hCursor.is_null());
        //
        // We want to load a standard system `IDC_*` cursor, so we pass `NULL` instead of our own `hExeInstance`.
        //
        // If we had embedded a custom cursor into our executable via an .rc file, and wanted to use that cursor, *then*
        // we would pass `hExeInstance`.  Alternatively, we could pass an `HMODULE` to a DLL if we wanted to load
        // cursors embedded in that DLL.
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw


        let wc = WNDCLASSW {
            lpfnWndProc: Some(sample_wndproc),
            hInstance: hExeInstance,
            hCursor,
            lpszClassName: wch_c!("SampleWndClass").as_ptr(),
            .. zeroed()
        };
        expect!(RegisterClassW(&wc) != 0);
        //
        // In windows, everything is a window.  Windows are windows, buttons are windows, textboxes are windows...
        // there's a reason Microsoft named it "Windows"!  For ease of reuse, Win32 first requires you to define a
        // window *class*, and then you can create multiple windows of that window class.
        //
        // Here, we're having our executable, `hExeInstance`, define a single window class, `SampleWndClass`, which is a
        // full blown top level window.  `hCursor` (`IDC_ARROW`) will be used when the cursor hovers over the window,
        // and `sample_wndproc` will be called whenever the window is resized, clicked, focused, typed into, redrawn...
        //
        // Proper Unicode support in many/most Win32 APIs requires null terminated UTF16 strings.  I'm using the
        // excellent `wchar::wch_c!` proc macro here which expands `wch_c!("Test").as_ptr()` into something like:
        //
        //      [b'T' as u16, b'e' as u16, b's' as u16, b't' as u16, b'\0' as u16].as_ptr()
        //
        // Note that this only works because the array is static.  If you used `vec![...]` instead, `lpszClassName`
        // would become a dangling pointer before `RegisterClassW` is called!  Even worse, unless you enable the Win32
        // debug heap, the code will usually work - meaning you won't know your code is a broken, ticking time bomb!
        // If you use dynamic strings/vecs, *name your temporaries and don't forget to add a 0*.  For example:
        //
        //      let lpszClassName = format!("SampleWndClass").encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        //      let wc = WNDCLASSW { lpszClassName: lpszClassName.as_ptr(), ... };
        //
        // Or for `std::ffi::OsStr`s:
        //
        //      use std::ffi::OsStr;
        //      use std::os::windows::prelude::*; // OsStrExt
        //      let lpszClassName = OsStr::new("SampleWndClass").encode_wide().chain(Some(0)).collect::<Vec<u16>>();
        //      let wc = WNDCLASSW { lpszClassName: lpszClassName.as_ptr(), ... };
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw
        // Ref:     https://docs.rs/wchar/0.6.1/wchar/macro.wch_c.html


        unsafe extern "system" fn sample_wndproc(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
            // Our sample wndproc is dreadfully boring:  it simply quits the program when the window is destroyed, and
            // uses default behavior for every other message.

            match uMsg {
                WM_DESTROY => {
                    // This message is recieved when the HWND is being destroyed.  This can occur even if you're not
                    // currently in a message loop / processing messages!  For example, if anything `DestroyWindow()`s
                    // your window, that can directly call this window proc.
                    //
                    // After it `WM_DESTROY` (and `WM_NCDESTROY`) have been recieved, it's no longer strictly sound to
                    // use the `HWND` in question, so this is a great spot to unregister HWNDs from any global lists you
                    // might be using to keep track of alive windows, and a great place to explode if you have scopes
                    // open that might expect those windows to be alive.  That said:  unwinding via Rust panic!()s over
                    // the Win32 FFI boundary is unsound as well, so consider either using panic = "abort" (for
                    // applications) or using some other, non-unwinding / instantly fatal error reporting mechanism (for
                    // libraries that register Win32 window classes.)
                    //
                    // MSDN:    https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy

                    PostQuitMessage(0);
                    //
                    // This simply posts a `WM_QUIT` message to the windows message loop.  While we could simply call
                    // `std::process::exit(0);`, using WM_QUIT gives the rest of the application a chance to save files,
                    // detect memory leaks, and otherwise more gracefully shut down.
                    //
                    // Since this application only ever has a single window, we probably want the application to exit if
                    // it's closed.  Multi-window applications might keep track of how many windows are open, and only
                    // call this when the last window has been destroyed.
                    //
                    // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage

                    0 // "If an application processes this message, it should return zero." (from WM_DESTROY's docs)
                },
                _ => {
                    DefWindowProcW(hwnd, uMsg, wParam, lParam)
                    //
                    // DefWindowProcW provides default behavior for most messages.  This function is 100% `unsafe` by
                    // Rust's definition of the keyword.  While `hwnd` hasn't been a real pointer in a long time[1][2],
                    // it once was, and `wParam`/`lParam` still frequently are.  When they are pointers, and what kinds
                    // of data they point to, is determined by the value of `uMsg`... and possibly by what class of
                    // window `hwnd` is for custom `WM_USER+N` messages.
                    //
                    // DefWindowProcW can and will dereference some of these pointers.  As such, `sample_wndproc` must
                    // be marked `unsafe` to be sound.  It's prudent to mark any window proc `unsafe`:  even if you can
                    // implement it soundly for the messages you're currently handling, handling new message types might
                    // require dereferencing raw pointers which would force you to make breaking API changes (adding
                    // `unsafe` later.)
                    //
                    // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw
                    // [1]      https://devblogs.microsoft.com/oldnewthing/20091210-00/?p=15713 "Only an idiot would have parameter validation, and only an idiot would not have it"
                    // [2]      https://devblogs.microsoft.com/oldnewthing/20070716-00/?p=26003 "How are window manager handles determined in 16-bit Windows and Windows 95?"
                },
            }
        }


        let hwnd = CreateWindowExW(
            0,                                  // extended style
            wch_c!("SampleWndClass").as_ptr(),  // window class
            wch_c!("Title").as_ptr(),           // window title
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,   // basic style
            CW_USEDEFAULT,                      // x
            CW_USEDEFAULT,                      // y
            800,                                // width
            600,                                // height
            null_mut(),                         // parent window
            null_mut(),                         // window menu bar
            hExeInstance,                       // instance containing the window class
            null_mut()                          // lpParam
        );
        expect!(!hwnd.is_null());
        //
        // This creates an actual top level window, using the window class we defined earlier.
        //
        // `WS_OVERLAPPEDWINDOW` is a style that's fairly typical for most top level windows - title bar, minimize and
        // maximize buttons, resizable border, etc.  Don't confuse this with `WS_OVERLAPPED`!
        //
        // `WS_VISIBLE` makes the window visible by default.  If not specified, we'd need to call `ShowWindow` later.
        // `CW_USEDEFAULT` lets the OS decide where to position the window, and can be used for sizes as well.
        // `hExeInstance` is specified to find `SampleWndClass` in the current executable.  If a DLL provided window
        // classes that we wanted to use instead, we'd provide an `hInstance` to that DLL instead.
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/winmsg/window-styles


        let mut rect : RECT = zeroed();
        expect!(GetClientRect(hwnd, &mut rect) != 0);
        let w = (rect.right - rect.left) as u32;
        let h = (rect.bottom - rect.top) as u32;
        //
        // We just created the window with a known size, so why are we querying the window for its size immediately?
        //
        // We specified the size for the *whole* window, including the border, titlebar, menubars, etc. - however, D3D
        // will only render to the "client area" of the window which is the inside portion of the window that *doesn't*
        // include the border, titlebar, etc. - so we need to figure out this smaller client size to create buffers of
        // matching size, that won't be squashed or cropped when displayed.
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect


        // Okay, we've reached the first bit of D3D11 code.  The first thing we need to do is create a whole slew of
        // core objects.  `D3D11CreateDeviceAndSwapChain`, which we'll call next, creates three of them:
        //
        //  1.  The swap chain.  This is just a set of one or more buffers - each describing the contents of an entire
        //      window or screen - and an interface for sending those buffers to the OS for them to be displayed.
        //      Multiple buffers are used to allow your application to modify/render one buffer, while the GPU or OS is
        //      busy displays the last fully completed buffer at the same time.  We can then *swap* between them once
        //      we're done rendering, and the OS is ready for a new frame.
        //
        //      https://en.wikipedia.org/wiki/Multiple_buffering
        //      https://docs.microsoft.com/en-us/windows/win32/direct3d9/what-is-a-swap-chain-
        //
        //  2.  The device.  This generally refers to a specific GPU, and is used as a factory to create most resources
        //      (textures, vertex buffers, shaders, etc).  There are a few cases where this can be yanked out from under
        //      you:  An external GPU can be unplugged, or the driver can be updated/hang/crash, or the GPU can hang.
        //
        //      To muddy the waters a bit:  Your window might be getting rendered with one GPU, but displayed on screens
        //      attached to another GPU.  And I'm not even sure what happens in multi-GPU SLI setups.
        //
        //      https://en.wikipedia.org/wiki/Scalable_Link_Interface
        //
        //  3.  A device context.  Device contexts accept various rendering commands and are full of tons of state.
        //      You always have exactly one "immediate" context per device - what you're asking the GPU to do right now.
        //      This is also the device context `D3D11CreateDeviceAndSwapChain` returns.
        //
        //      Optionally, you can also create "deferred" contexts, which you can later run on the immediate context.
        //      These are mainly intended to enable multithreaded rendering, but might also be useful for isolating
        //      middleware such that it's changes to the device context's state don't break your own rendering code.
        //
        // In Direct3D 9, "device" and "device context" were conflated concepts.
        // In OpenGL, all three objects are vaguely conflated together into the concept of an "OpenGL Context".


        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width:              w,
                Height:             h,
                RefreshRate:        zeroed(),                               // Requested refresh rate (AFAIK, this is only used for fullscreen modes)
                Format:             DXGI_FORMAT_R8G8B8A8_UNORM,             // https://docs.microsoft.com/en-us/windows/win32/api/dxgiformat/ne-dxgiformat-dxgi_format
                ScanlineOrdering:   DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,   // https://docs.microsoft.com/en-us/previous-versions/windows/desktop/legacy/bb173067(v=vs.85) / https://en.wikipedia.org/wiki/Interlaced_video
                Scaling:            DXGI_MODE_SCALING_CENTERED,             // How to re-scale the buffers to the window's client area (if mismatched)
            },
            SampleDesc:     DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },  // No multisampling ( https://en.wikipedia.org/wiki/Multisample_anti-aliasing )
            BufferUsage:    DXGI_USAGE_RENDER_TARGET_OUTPUT,            // Required flag
            BufferCount:    1,                          // Only use a single buffer.  We presumably will be blocked from rendering when Windows is rendering our window.
            OutputWindow:   hwnd,                       // Render to our window
            Windowed:       1,                          // Stay as a window instead of entering fullscreen mode
            SwapEffect:     DXGI_SWAP_EFFECT_DISCARD,   // Fastest option - assumes we'll redraw everything.  https://docs.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_swap_effect
            Flags:          0,                          // https://docs.microsoft.com/en-us/windows/win32/api/dxgi/ne-dxgi-dxgi_swap_chain_flag
        };
        //
        // Describe what settings we want to use for the swap chain.  Some settings are ignored for windowed modes.
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/dxgi/ns-dxgi-dxgi_swap_chain_desc


        let feature_levels = &[D3D_FEATURE_LEVEL_11_0];
        //
        // "Feature Levels" are Direct3D 11's way of letting you select what generation of GPU hardware you want to
        // target.  You can use GPUs which "only" support "Direct3D 10" from Direct3D 11 by using D3D_FEATURE_LEVEL_10_0.
        // Unless your rendering code provides multiple codepaths for newer vs older GPUs, you probably only need to
        // specify a single feature level - the minimum feature level you support.
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_feature_level


        let mut swap_chain = null_mut();
        let mut device = null_mut();
        let mut device_context = null_mut();
        expect!(SUCCEEDED(D3D11CreateDeviceAndSwapChain(
            // Inputs
            null_mut(),                 // adapter - you might want to let the user specify which display/GPU to use through this
            D3D_DRIVER_TYPE_HARDWARE,   // https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_driver_type
            null_mut(),                 // software - you almost certainly don't want to use this!  Used only for *custom* software drivers, and is supposedly quite slow.
            0,                          // https://docs.microsoft.com/en-us/windows/win32/api/d3d11/ne-d3d11-d3d11_create_device_flag
            feature_levels.as_ptr(),
            feature_levels.len() as u32,
            D3D11_SDK_VERSION,          // magic constant so d3d11.dll knows what header(s) you built with / `winapi` was designed to use
            &swap_chain_desc,
            // Outputs
            &mut swap_chain,
            &mut device,
            null_mut(),                 // the selected level from `feature_levels` (e.g. `D3D_FEATURE_LEVEL_11_0`)
            &mut device_context
        )));
        let swap_chain      = mcom::Rc::from_raw(swap_chain);
        let device          = mcom::Rc::from_raw(device);
        let device_context  = mcom::Rc::from_raw(device_context);
        //
        // `swap_chain`, `device`, and `device_context` are all intrusively refcounted COM objects (or close enough).
        //
        // The basic rule of thumb is that any method with COM objects as out-parameters either creates the object with
        // an initial refcount of 1, or increments the refcount by 1 if the object already existed.  As such, all such
        // calls should eventually be paired with a corresponding `(*com_object).Release()` call to avoid memory leaks.
        //
        // Here, I'm using `mcom::Rc`, which will automatically call `Release()` when dropped.  It also provides a
        // `Deref` implementation, so I can write `com_object.Method()` instead of spamming `(*com_object).Method()`.
        //
        // MSDN:    https://docs.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-d3d11createdeviceandswapchain
        // Ref:     https://docs.rs/mcom/0.1.1/mcom/struct.Rc.html#method.from_raw
        // Alt:     https://docs.rs/wio/0.2.2/x86_64-pc-windows-msvc/wio/com/struct.ComPtr.html


        let mut back_buffer = null_mut();
        expect!(SUCCEEDED(swap_chain.GetBuffer(0, &ID3D11Resource::uuidof(), &mut back_buffer)));
        let back_buffer = mcom::Rc::from_raw(back_buffer as *mut ID3D11Resource);

        let mut rtv = null_mut();
        expect!(SUCCEEDED(device.CreateRenderTargetView(back_buffer.as_ptr(), null_mut(), &mut rtv)));
        let rtv = mcom::Rc::from_raw(rtv);

        device_context.OMSetRenderTargets(1, [rtv.as_ptr()].as_ptr(), null_mut());

        let vp = D3D11_VIEWPORT { Width: w as f32, Height: h as f32, MinDepth: 0.0, MaxDepth: 1.0, TopLeftX: 0.0, TopLeftY: 0.0 };
        device_context.RSSetViewports(1, [vp].as_ptr());

        let vs_bin = include_bytes!("../target/assets/vs.bin");
        let ps_bin = include_bytes!("../target/assets/ps.bin");
        let mut vs = null_mut();
        let mut ps = null_mut();
        expect!(SUCCEEDED((*device).CreateVertexShader(vs_bin.as_ptr() as *const _, vs_bin.len(), null_mut(), &mut vs)));
        expect!(SUCCEEDED((*device).CreatePixelShader( ps_bin.as_ptr() as *const _, ps_bin.len(), null_mut(), &mut ps)));
        let vs = mcom::Rc::from_raw(vs);
        let ps = mcom::Rc::from_raw(ps);

        let mut input_layout = null_mut();
        expect!(SUCCEEDED((*device).CreateInputLayout(SimpleVertex::layout().as_ptr() as *const _, SimpleVertex::layout().len() as UINT, vs_bin.as_ptr() as *const _, vs_bin.len(), &mut input_layout)));
        let input_layout = mcom::Rc::from_raw(input_layout);

        let verticies = [
            SimpleVertex::new(Vector::new( 0.0,  0.5, 0.5, 0.0)),
            SimpleVertex::new(Vector::new( 0.5, -0.5, 0.5, 0.0)),
            SimpleVertex::new(Vector::new(-0.5, -0.5, 0.5, 0.0)),
        ];

        let bd = D3D11_BUFFER_DESC {
            Usage:              D3D11_USAGE_DEFAULT,
            ByteWidth:          size_of_val(&verticies) as UINT,
            BindFlags:          D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags:     0,
            MiscFlags:          0,
            .. zeroed()
        };

        let init_data = D3D11_SUBRESOURCE_DATA {
            pSysMem: verticies.as_ptr() as *const _,
            .. zeroed()
        };

        let mut vertex_buffer = null_mut();
        expect!(SUCCEEDED(device.CreateBuffer(&bd, &init_data, &mut vertex_buffer)));
        let vertex_buffer = mcom::Rc::from_raw(vertex_buffer);

        loop {
            let mut msg : MSG = zeroed();
            while PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT { return; }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            device_context.ClearRenderTargetView(rtv.as_ptr(), &[0.1, 0.2, 0.3, 1.0]);
            device_context.IASetInputLayout(input_layout.as_ptr());
            device_context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            device_context.IASetVertexBuffers(0, 1, [vertex_buffer.as_ptr()].as_ptr(), [size_of::<SimpleVertex>() as UINT].as_ptr(), [0].as_ptr());
            device_context.VSSetShader(vs.as_ptr(), null_mut(), 0);
            device_context.PSSetShader(ps.as_ptr(), null_mut(), 0);
            device_context.Draw(3, 0);
            swap_chain.Present(0, 0);
        }
    };
}


#[repr(transparent)]
#[derive(Clone, Copy)]
struct InputElementDesc<'a>(D3D11_INPUT_ELEMENT_DESC, PhantomData<&'a str>);
unsafe impl<'a> Sync for InputElementDesc<'a> {}

macro_rules! input_layout {
    ($({ $semantic_name:expr , $semantic_index:expr , $format:expr , $input_slot:expr , $aligned_byte_offset:expr , $input_slot_class:expr , $instance_data_step_rate:expr }),+ $(,)?) => {
        [
            $(InputElementDesc(D3D11_INPUT_ELEMENT_DESC {
                SemanticName:           concat!($semantic_name, "\0").as_ptr() as *const _,
                SemanticIndex:          $semantic_index,
                Format:                 $format,
                InputSlot:              $input_slot,
                AlignedByteOffset:      $aligned_byte_offset,
                InputSlotClass:         $input_slot_class,
                InstanceDataStepRate:   $instance_data_step_rate,
            }, PhantomData)),+
        ]
    };
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector {
    fn new (x: f32, y: f32, z: f32, w: f32) -> Self { Self {x, y, z, w} }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct SimpleVertex {
    pub pos: Vector,
}

impl SimpleVertex {
    fn new (pos: Vector) -> Self { Self { pos } }

    fn layout() -> &'static [InputElementDesc<'static>] {
        static LAYOUT : [InputElementDesc; 1] = input_layout! {
            { "POSITION", 0, DXGI_FORMAT_R32G32B32_FLOAT, 0,  0, D3D11_INPUT_PER_VERTEX_DATA, 0 },
        };
        &LAYOUT[..]
    }
}

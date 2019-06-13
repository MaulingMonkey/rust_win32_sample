// Vaguely adopted from https://docs.microsoft.com/en-us/windows/desktop/learnwin32/your-first-windows-program
// ...and https://code.msdn.microsoft.com/windowsdesktop/Direct3D-Tutorial-Win32-829979ef
//     1) https://docs.microsoft.com/en-us/previous-versions//ff729718(v=vs.85)
//     2) https://docs.microsoft.com/en-us/previous-versions//ff729719(v=vs.85)
//     3) https://docs.microsoft.com/en-us/previous-versions//ff729720(v=vs.85)

#![windows_subsystem = "windows"]
#![allow(non_snake_case)] // WinAPI style

mod debug;
#[macro_use] mod macros;
mod win32;
mod window;

use window::*;
use win32::*;
use std::{ptr, mem};
use std::marker::{PhantomData};

#[repr(transparent)]
#[derive(Clone, Copy)]
struct InputElementDesc<'a>(D3D11_INPUT_ELEMENT_DESC, PhantomData<&'a str>);
unsafe impl<'a> Sync for InputElementDesc<'a> {}

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

fn main() {
    debug::init();
    let mut window = Window::new();
    window.show();
    let client = window.client_area();

    unsafe {
        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
            BufferDesc: DXGI_MODE_DESC {
                Width:  client.width(),
                Height: client.height(),
                RefreshRate: DXGI_RATIONAL { Numerator: 60, Denominator: 1 },
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
                Scaling: DXGI_MODE_SCALING_CENTERED,
            },
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            OutputWindow: window.hwnd(),
            Windowed: 1,
            SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
            Flags: 0,
        };

        let mut swap_chain = ptr::null_mut();
        let mut device = ptr::null_mut();
        let mut device_context = ptr::null_mut();
        let feature_levels = &[D3D_FEATURE_LEVEL_11_0];
        expect!(SUCCEEDED(D3D11CreateDeviceAndSwapChain(
            ptr::null_mut(), // adapter
            D3D_DRIVER_TYPE_HARDWARE,
            ptr::null_mut(), // software
            0, // flags
            feature_levels.as_ptr(),
            feature_levels.len() as u32,
            D3D11_SDK_VERSION,
            &swap_chain_desc,
            &mut swap_chain,
            &mut device,
            ptr::null_mut(),
            &mut device_context
        )));
        expect_ne!(swap_chain,     ptr::null_mut());
        expect_ne!(device,         ptr::null_mut());
        expect_ne!(device_context, ptr::null_mut());
        let swap_chain      = &*swap_chain;
        let device          = &*device;
        let device_context  = &*device_context;

        let mut back_buffer = ptr::null_mut();
        expect!(SUCCEEDED(swap_chain.GetBuffer(0, &ID3D11Texture2D::uuidof(), &mut back_buffer)));
        let back_buffer = back_buffer as * mut _;

        let mut rtv = ptr::null_mut();
        expect!(SUCCEEDED(device.CreateRenderTargetView(back_buffer, ptr::null_mut(), &mut rtv)));
        expect_ne!(rtv, ptr::null_mut());

        device_context.OMSetRenderTargets(1, [rtv].as_ptr(), ptr::null_mut());

        let vp = D3D11_VIEWPORT { Width: client.width() as f32, Height: client.height() as f32, MinDepth: 0.0, MaxDepth: 1.0, TopLeftX: 0.0, TopLeftY: 0.0 };
        device_context.RSSetViewports(1, [vp].as_ptr());

        let vs_bin = include_bytes!("../target/assets/vs.bin");
        let ps_bin = include_bytes!("../target/assets/ps.bin");
        let mut vs = ptr::null_mut();
        let mut ps = ptr::null_mut();
        expect!(SUCCEEDED(device.CreateVertexShader(vs_bin.as_ptr() as *const _, vs_bin.len(), ptr::null_mut(), &mut vs)));
        expect!(SUCCEEDED(device.CreatePixelShader( ps_bin.as_ptr() as *const _, ps_bin.len(), ptr::null_mut(), &mut ps)));
        let mut input_layout = ptr::null_mut();
        expect!(SUCCEEDED(device.CreateInputLayout(SimpleVertex::layout().as_ptr() as *const _, SimpleVertex::layout().len() as UINT, vs_bin.as_ptr() as *const _, vs_bin.len(), &mut input_layout)));

        let verticies = [
            SimpleVertex::new(Vector::new( 0.0,  0.5, 0.5, 0.0)),
            SimpleVertex::new(Vector::new( 0.5, -0.5, 0.5, 0.0)),
            SimpleVertex::new(Vector::new(-0.5, -0.5, 0.5, 0.0)),
        ];

        let bd = D3D11_BUFFER_DESC {
            Usage:              D3D11_USAGE_DEFAULT,
            ByteWidth:          mem::size_of_val(&verticies) as UINT,
            BindFlags:          D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags:     0,
            MiscFlags:          0,
            .. mem::zeroed()
        };

        let init_data = D3D11_SUBRESOURCE_DATA {
            pSysMem: verticies.as_ptr() as *const _,
            .. mem::zeroed()
        };

        let mut vertex_buffer = ptr::null_mut();
        expect!(SUCCEEDED(device.CreateBuffer(&bd, &init_data, &mut vertex_buffer)));

        loop {
            expect!(window.is_alive());
            let mut msg : MSG = mem::zeroed();
            while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT {
                    expect!(!window.is_alive());
                    return;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            device_context.ClearRenderTargetView(rtv, &[0.1, 0.2, 0.3, 1.0]);
            device_context.IASetInputLayout(input_layout);
            device_context.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            device_context.IASetVertexBuffers(0, 1, [vertex_buffer].as_ptr(), [mem::size_of::<SimpleVertex>() as UINT].as_ptr(), [0].as_ptr());
            device_context.VSSetShader(vs, ptr::null_mut(), 0);
            device_context.PSSetShader(ps, ptr::null_mut(), 0);
            device_context.Draw(3, 0);
            swap_chain.Present(0, 0);
        }
    };
}

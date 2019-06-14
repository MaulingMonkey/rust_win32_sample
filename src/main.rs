// Vaguely adopted from https://docs.microsoft.com/en-us/windows/desktop/learnwin32/your-first-windows-program
// ...and https://code.msdn.microsoft.com/windowsdesktop/Direct3D-Tutorial-Win32-829979ef
//     1) https://docs.microsoft.com/en-us/previous-versions//ff729718(v=vs.85)
//     2) https://docs.microsoft.com/en-us/previous-versions//ff729719(v=vs.85)
//     3) https://docs.microsoft.com/en-us/previous-versions//ff729720(v=vs.85)

#![windows_subsystem = "windows"]
#![allow(non_snake_case)] // WinAPI style

#[macro_use] mod macros;
mod com;
mod debug;
mod win32;
mod window;

use com::d3d11;
use com::*;
use window::*;
use win32::*;
use std::{ptr, mem};
use std::marker::{PhantomData};
use std::convert::AsRef;

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

    fn layout() -> &'static [d3d11::InputElementDesc<'static>] {
        static LAYOUT : [d3d11::InputElementDesc; 1] = input_layout! {
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

    let dasc = unsafe { DeviceAndSwapChain::create(
        None, // adapter
        d3d11::DriverType::Hardware,
        None, // software
        0, // flags
        Some(&[d3d11::FeatureLevel::_11_0]),
        &swap_chain_desc
    )}.unwrap();

    let swap_chain      = &dasc.swap_chain;
    let device          = &dasc.device;
    let device_context  = &dasc.device_context;

    let back_buffer = swap_chain.get_buffer::<d3d11::Texture2D>(0).unwrap();
    let rtv = device.create_render_target_view(&back_buffer, None).unwrap();

    device_context.om_set_render_targets(&[rtv.as_ref()], None);

    let vp = D3D11_VIEWPORT { Width: client.width() as f32, Height: client.height() as f32, MinDepth: 0.0, MaxDepth: 1.0, TopLeftX: 0.0, TopLeftY: 0.0 };
    device_context.rs_set_viewports(&[vp]);

    let vs_bin = include_bytes!("../target/assets/vs.bin");
    let ps_bin = include_bytes!("../target/assets/ps.bin");
    let vs = device.create_vertex_shader(vs_bin, None).unwrap();
    let ps = device.create_pixel_shader(ps_bin, None).unwrap();
    let input_layout = device.create_input_layout(SimpleVertex::layout(), vs_bin).unwrap();

    let verticies = [
        SimpleVertex::new(Vector::new( 0.0,  0.5, 0.5, 0.0)),
        SimpleVertex::new(Vector::new( 0.5, -0.5, 0.5, 0.0)),
        SimpleVertex::new(Vector::new(-0.5, -0.5, 0.5, 0.0)),
    ];

    let bd = D3D11_BUFFER_DESC {
        ByteWidth:              mem::size_of_val(&verticies) as UINT,
        Usage:                  D3D11_USAGE_DEFAULT,
        BindFlags:              D3D11_BIND_VERTEX_BUFFER,
        CPUAccessFlags:         0,
        MiscFlags:              0,
        StructureByteStride:    0,
    };

    let init_data = D3D11_SUBRESOURCE_DATA {
        pSysMem:            verticies.as_ptr() as *const _,
        SysMemPitch:        0,
        SysMemSlicePitch:   0,
    };

    let vertex_buffer = unsafe { device.create_buffer(&bd, Some(&init_data)) }.unwrap();

    loop {
        expect!(window.is_alive());
        unsafe {
            let mut msg : MSG = mem::zeroed();
            while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_QUIT {
                    expect!(!window.is_alive());
                    return;
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        device_context.clear_render_target_view(&rtv, &[0.1, 0.2, 0.3, 1.0]);
        device_context.ia_set_input_layout(&input_layout);
        device_context.ia_set_primitive_topology(d3d11::PrimitiveTopology::TriangleList);
        device_context.ia_set_vertex_buffers(0, &[vertex_buffer.as_ref()], &[mem::size_of::<SimpleVertex>() as UINT], &[0]);
        device_context.vs_set_shader(&vs, &[]);
        device_context.ps_set_shader(&ps, &[]);
        device_context.draw(3, 0);
        swap_chain.present(0, 0).unwrap();
    }
}

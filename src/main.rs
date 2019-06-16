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

use com::d3d;
use com::d3d11;
use com::*;
use win32::*;
use std::fs;
use std::mem;
use std::marker::{PhantomData};
use std::path::{Path, PathBuf};
use std::ffi::{CStr};
use std::convert::AsRef;
use winit::window::*;
use winit::event::*;
use winit::event_loop::*;
use winit::platform::windows::WindowExtWindows;

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

fn require_shader (name: &Path, target: d3d::Target) -> Vec<u8> {
    let src_path = PathBuf::from("res").join(name);
    let bin_path = PathBuf::from(r"target\assets").join(name).with_extension("bin");
    let bin_dir = bin_path.parent().unwrap();
    if !bin_dir.exists() {
        fs::create_dir_all(&bin_dir).unwrap();
    }
    if !bin_path.exists() {
        let result = unsafe { d3d::compile(
            &fs::read(&src_path).unwrap()[..],
            Some(&src_path),
            None, // Defines
            Some(d3d::COMPILE_STANDARD_FILE_INCLUDE),
            Some(CStr::from_bytes_with_nul(b"main\0").unwrap()),
            target.to_cstr(),
            0, // flags1
            0  // flags2
        )}.unwrap();
        
        result.shader.write_file(&bin_path, false).unwrap();
    }
    fs::read(bin_path).unwrap()
}

fn main() {
    debug::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Win32 Sample")
        .with_inner_size((800, 600).into())
        .build(&event_loop)
        .unwrap();
    let client = window.inner_size();

    let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: DXGI_MODE_DESC {
            Width:  client.width as UINT,
            Height: client.height as UINT,
            RefreshRate: DXGI_RATIONAL { Numerator: 60, Denominator: 1 },
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
            Scaling: DXGI_MODE_SCALING_CENTERED,
        },
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 1,
        OutputWindow: window.hwnd() as HWND,
        Windowed: 1,
        SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
        Flags: 0,
    };

    let DeviceAndSwapChain { swap_chain, device, device_context, .. } = unsafe { DeviceAndSwapChain::create(
        None, // adapter
        d3d11::DriverType::Hardware,
        None, // software
        0, // flags
        Some(&[d3d11::FeatureLevel::_11_0]),
        &swap_chain_desc
    )}.unwrap();

    let back_buffer = swap_chain.get_buffer::<d3d11::Texture2D>(0).unwrap();
    let rtv = device.create_render_target_view(&back_buffer, None).unwrap();

    device_context.om_set_render_targets(&[rtv.as_ref()], None);

    let vp = D3D11_VIEWPORT { Width: client.width as f32, Height: client.height as f32, MinDepth: 0.0, MaxDepth: 1.0, TopLeftX: 0.0, TopLeftY: 0.0 };
    device_context.rs_set_viewports(&[vp]);

    let vs_bin = require_shader(Path::new("vs.hlsl"), d3d::Target::vs_5_0);
    let ps_bin = require_shader(Path::new("ps.hlsl"), d3d::Target::ps_5_0);
    let vs = device.create_vertex_shader(&vs_bin[..], None).unwrap();
    let ps = device.create_pixel_shader(&ps_bin[..], None).unwrap();
    let input_layout = device.create_input_layout(SimpleVertex::layout(), &vs_bin[..]).unwrap();

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
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,
                Event::EventsCleared => {
                    device_context.clear_render_target_view(&rtv, &[0.1, 0.2, 0.3, 1.0]);
                    device_context.ia_set_input_layout(&input_layout);
                    device_context.ia_set_primitive_topology(d3d11::PrimitiveTopology::TriangleList);
                    device_context.ia_set_vertex_buffers(0, &[vertex_buffer.as_ref()], &[mem::size_of::<SimpleVertex>() as UINT], &[0]);
                    device_context.vs_set_shader(&vs, &[]);
                    device_context.ps_set_shader(&ps, &[]);
                    device_context.draw(3, 0);
                    swap_chain.present(0, 0).unwrap();
                },
                _ => {},
            }
        });
    }
}

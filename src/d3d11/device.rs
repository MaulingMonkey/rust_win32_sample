use crate::d3d11::*;
use std::ptr::{null, null_mut};

pub struct Device {
    device: *const ID3D11Device,
}

impl Clone for Device {
    fn clone (&self) -> Self {
        unsafe { (*self.device).AddRef() };
        Self { device: self.device }
    }
}

impl Drop for Device {
    fn drop (&mut self) {
        unsafe { (*self.device).Release() };
    }
}

impl Into<*const ID3D11Device> for &Device {
    fn into (self) -> *const ID3D11Device { self.device }
}

pub trait IntoResource {
    fn into_resource(self) -> *mut ID3D11Resource;
}

impl IntoResource for *mut ID3D11Resource  { fn into_resource(self) -> *mut ID3D11Resource { self } }
impl IntoResource for *mut ID3D11Texture2D { fn into_resource(self) -> *mut ID3D11Resource { self as *mut _ } }

impl Device {
    /// Take ownership of a ID3D11Device
    pub unsafe fn from (device: *const ID3D11Device) -> Option<Self> {
        if device == null() {
            None
        } else {
            Some(Self { device })
        }
    }

    /// Create a buffer.  `initial_data` cannot be `None` if `desc.Usage == D3D11_USAGE_IMMUTABLE`.
    pub unsafe fn create_buffer (&self, desc: &D3D11_BUFFER_DESC, initial_data: Option<&D3D11_SUBRESOURCE_DATA>) -> Result<*mut ID3D11Buffer, HRESULT> {
        let mut buffer = null_mut();
        let result = (*self.device).CreateBuffer(
            &*desc,
            initial_data.map_or(null(), |id| &*id),
            &mut buffer
        );
        if SUCCEEDED(result) {
            expect_ne!(buffer, null_mut());
            Ok(buffer)
        } else {
            Err(result)
        }
    }

    pub fn create_render_target_view<D: IntoResource> (&self, resource: D, desc: Option<&D3D11_RENDER_TARGET_VIEW_DESC>) -> Result<*mut ID3D11RenderTargetView, HRESULT> {
        let mut rtv = null_mut();
        let result = unsafe { (*self.device).CreateRenderTargetView(
            resource.into_resource(),
            desc.map_or(null(), |d| &*d),
            &mut rtv
        )};
        if SUCCEEDED(result) {
            expect_ne!(rtv, null_mut());
            Ok(rtv)
        } else {
            Err(result)
        }
    }

    pub fn create_vertex_shader (&self, bytecode: &[u8], class_linkage: *mut ID3D11ClassLinkage) -> Result<*mut ID3D11VertexShader, HRESULT> {
        let mut vs = null_mut();
        let result = unsafe { (*self.device).CreateVertexShader(
            bytecode.as_ptr() as *const _,
            bytecode.len() as SIZE_T,
            class_linkage,
            &mut vs
        )};
        if SUCCEEDED(result) {
            expect_ne!(vs, null_mut());
            Ok(vs)
        } else {
            Err(result)
        }
    }

    pub fn create_pixel_shader (&self, bytecode: &[u8], class_linkage: *mut ID3D11ClassLinkage) -> Result<*mut ID3D11PixelShader, HRESULT> {
        let mut ps = null_mut();
        let result = unsafe { (*self.device).CreatePixelShader(
            bytecode.as_ptr() as *const _,
            bytecode.len() as SIZE_T,
            class_linkage,
            &mut ps
        )};
        if SUCCEEDED(result) {
            expect_ne!(ps, null_mut());
            Ok(ps)
        } else {
            Err(result)
        }
    }

    pub fn create_input_layout<I: IntoInputElements> (&self, input_element_descs: I, shader_bytecode_with_input_signature: &[u8]) -> Result<*mut ID3D11InputLayout, HRESULT> {
        let input_element_descs = input_element_descs.into_input_elements();
        let mut il = null_mut();
        let result = unsafe { (*self.device).CreateInputLayout(
            input_element_descs.as_ptr(),
            input_element_descs.len() as UINT,
            shader_bytecode_with_input_signature.as_ptr() as *const _,
            shader_bytecode_with_input_signature.len() as SIZE_T,
            &mut il
        )};
        if SUCCEEDED(result) {
            expect_ne!(il, null_mut());
            Ok(il)
        } else {
            Err(result)
        }
    }
}

use crate::com::d3d11::*;
use crate::com::Wrapper;
use std::ptr::{null, null_mut};

com_wrapper!{ pub struct Device(*mut ID3D11Device); }

pub trait IntoResource { fn into_resource(self) -> *mut ID3D11Resource; }
impl IntoResource for *mut ID3D11Resource  { fn into_resource(self) -> *mut ID3D11Resource { self } }
impl IntoResource for *mut ID3D11Texture2D { fn into_resource(self) -> *mut ID3D11Resource { self as *mut _ } }
impl IntoResource for &Texture2D           { fn into_resource(self) -> *mut ID3D11Resource { &**self.as_ref() as *const _ as *mut _ } } // XXX

/// MSDN: [ID3D11Device](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nn-d3d11-id3d11device)
impl Device {
    /// MSDN: [ID3D11Device::CreateBuffer](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11device-createbuffer)
    /// 
    /// Create a buffer.  `initial_data` cannot be `None` if `desc.Usage == D3D11_USAGE_IMMUTABLE`.
    /// 
    /// `unsafe`:  `D3D11_SUBRESOURCE_DATA::pSysMem` may not be valid & may be dereferenced.
    pub unsafe fn create_buffer (&self, desc: &D3D11_BUFFER_DESC, initial_data: Option<&D3D11_SUBRESOURCE_DATA>) -> Result<Buffer, HRESULT> {
        let mut buffer = null_mut();
        let result = self.as_ref().CreateBuffer(
            &*desc,
            initial_data.map_or(null(), |id| &*id),
            &mut buffer
        );
        if SUCCEEDED(result) {
            Ok(Buffer::own(buffer).unwrap())
        } else {
            Err(result)
        }
    }

    /// MSDN: [ID3D11Device::CreateRenderTargetView](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11device-createrendertargetview)
    pub fn create_render_target_view<D: IntoResource> (&self, resource: D, desc: Option<&D3D11_RENDER_TARGET_VIEW_DESC>) -> Result<RenderTargetView, HRESULT> {
        let mut rtv = null_mut();
        let result = unsafe { self.as_ref().CreateRenderTargetView(
            resource.into_resource(),
            desc.map_or(null(), |d| &*d),
            &mut rtv
        )};
        if SUCCEEDED(result) {
            Ok(unsafe { RenderTargetView::own(rtv) }.unwrap())
        } else {
            Err(result)
        }
    }

    /// MSDN: [ID3D11Device::CreateVertexShader](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11device-createvertexshader)
    pub fn create_vertex_shader (&self, bytecode: &[u8], class_linkage: Option<&ClassLinkage>) -> Result<VertexShader, HRESULT> {
        let mut vs = null_mut();
        let result = unsafe { self.as_ref().CreateVertexShader(
            bytecode.as_ptr() as *const _,
            bytecode.len() as SIZE_T,
            class_linkage.map_or(null_mut(), |cl| cl.as_ptr()),
            &mut vs
        )};
        if SUCCEEDED(result) {
            Ok(unsafe { VertexShader::own(vs) }.unwrap())
        } else {
            Err(result)
        }
    }

    /// MSDN: [ID3D11Device::CreatePixelShader](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11device-createpixelshader)
    pub fn create_pixel_shader (&self, bytecode: &[u8], class_linkage: Option<&ClassLinkage>) -> Result<PixelShader, HRESULT> {
        let mut ps = null_mut();
        let result = unsafe { self.as_ref().CreatePixelShader(
            bytecode.as_ptr() as *const _,
            bytecode.len() as SIZE_T,
            class_linkage.map_or(null_mut(), |cl| cl.as_ptr()),
            &mut ps
        )};
        if SUCCEEDED(result) {
            Ok(unsafe { PixelShader::own(ps) }.unwrap())
        } else {
            Err(result)
        }
    }

    /// MSDN: [ID3D11Device::CreateInputLayout](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11device-createinputlayout)
    pub fn create_input_layout<I: IntoInputElements> (&self, input_element_descs: I, shader_bytecode_with_input_signature: &[u8]) -> Result<InputLayout, HRESULT> {
        let input_element_descs = input_element_descs.into_input_elements();
        let mut il = null_mut();
        let result = unsafe { self.as_ref().CreateInputLayout(
            input_element_descs.as_ptr(),
            input_element_descs.len() as UINT,
            shader_bytecode_with_input_signature.as_ptr() as *const _,
            shader_bytecode_with_input_signature.len() as SIZE_T,
            &mut il
        )};
        if SUCCEEDED(result) {
            Ok(unsafe { InputLayout::own(il) }.unwrap())
        } else {
            Err(result)
        }
    }
}

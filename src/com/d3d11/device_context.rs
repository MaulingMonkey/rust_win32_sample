use crate::com::*;
use crate::com::d3d11::*;
use std::ptr::{null_mut};

com_wrapper!{ pub struct DeviceContext(*mut ID3D11DeviceContext); }

/// MSDN: [ID3D11DeviceContext](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nn-d3d11-id3d11devicecontext)
impl DeviceContext {
    /// MSDN: [ID3D11DeviceContext::OMSetRenderTargets](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-omsetrendertargets)
    pub fn om_set_render_targets (&self, render_target_views: &[&ID3D11RenderTargetView], depth_stencil_view: Option<&DepthStencilView>) {
        unsafe {
            let render_target_views = render_target_views.as_native_slice();
            self.as_ref().OMSetRenderTargets(
                render_target_views.len() as UINT,
                render_target_views.as_ptr(),
                depth_stencil_view.map_or(null_mut(), |dsv| dsv.as_ptr())
            );
        }
    }

    /// MSDN: [ID3D11DeviceContext::RSSetViewports](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-rssetviewports)
    pub fn rs_set_viewports (&self, viewports: &[D3D11_VIEWPORT]) {
        unsafe { self.as_ref().RSSetViewports(
            viewports.len() as UINT,
            viewports.as_ptr()
        )};
    }

    /// MSDN: [ID3D11DeviceContext::ClearRenderTargetView](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-clearrendertargetview)
    pub fn clear_render_target_view (&self, render_target_view: &RenderTargetView, rgba: &[FLOAT; 4]) {
        unsafe { self.as_ref().ClearRenderTargetView(
            render_target_view.as_ptr(),
            rgba
        )};
    }

    /// MSDN: [ID3D11DeviceContext::IASetInputLayout](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-iasetinputlayout)
    pub fn ia_set_input_layout (&self, input_layout: &InputLayout) {
        unsafe { self.as_ref().IASetInputLayout(input_layout.as_ptr()) };
    }

    /// MSDN: [ID3D11DeviceContext::IASetPrimitiveTopology](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-iasetprimitivetopology)
    pub fn ia_set_primitive_topology (&self, topology: PrimitiveTopology) {
        unsafe { self.as_ref().IASetPrimitiveTopology(topology.raw()) };
    }

    /// MSDN: [ID3D11DeviceContext::IASetVertexBuffers](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-iasetvertexbuffers)
    pub fn ia_set_vertex_buffers (
        &self,
        start_slot:     UINT,
        vertex_buffers: &[&ID3D11Buffer],
        strides:        &[UINT],
        offsets:        &[UINT]
    ) {
        expect_eq!(vertex_buffers.len(), strides.len());
        expect_eq!(vertex_buffers.len(), offsets.len());
        let n = vertex_buffers.len().min(strides.len()).min(offsets.len());

        unsafe {
            let vertex_buffers = vertex_buffers.as_native_slice();
            self.as_ref().IASetVertexBuffers(start_slot, n as UINT, vertex_buffers.as_ptr(), strides.as_ptr(), offsets.as_ptr())
        };
    }

    /// MSDN: [ID3D11DeviceContext::VSSetShader](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-vssetshader)
    pub fn vs_set_shader (&self, vertex_shader: &VertexShader, class_instances: &[Option<&ID3D11ClassInstance>]) {
        unsafe {
            let class_instances = class_instances.as_native_slice();
            self.as_ref().VSSetShader(vertex_shader.as_ptr(), class_instances.as_ptr(), class_instances.len() as UINT)
        };
    }

    /// MSDN: [ID3D11DeviceContext::PSSetShader](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-pssetshader)
    pub fn ps_set_shader (&self, pixel_shader: &PixelShader, class_instances: &[Option<&ID3D11ClassInstance>]) {
        unsafe {
            let class_instances = class_instances.as_native_slice();
            self.as_ref().PSSetShader(pixel_shader.as_ptr(), class_instances.as_ptr(), class_instances.len() as UINT)
        };
    }

    /// MSDN: [ID3D11DeviceContext::Draw](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-id3d11devicecontext-draw)
    pub fn draw (&self, vertex_count: UINT, start_vertex_location: UINT) {
        unsafe { self.as_ref().Draw(vertex_count, start_vertex_location) };
    }
}

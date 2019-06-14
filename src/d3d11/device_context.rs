use crate::d3d11::*;
use std::ptr::{null};

pub struct DeviceContext {
    device_context: *const ID3D11DeviceContext,
}

impl Clone for DeviceContext {
    fn clone (&self) -> Self {
        unsafe { (*self.device_context).AddRef() };
        Self { device_context: self.device_context }
    }
}

impl Drop for DeviceContext {
    fn drop (&mut self) {
        unsafe { (*self.device_context).Release() };
    }
}

impl Into<*const ID3D11DeviceContext> for &DeviceContext {
    fn into (self) -> *const ID3D11DeviceContext { self.device_context }
}

impl DeviceContext {
    /// Take ownership of a ID3D11Device
    pub unsafe fn from (device_context: *const ID3D11DeviceContext) -> Option<Self> {
        if device_context == null() {
            None
        } else {
            Some(Self { device_context })
        }
    }

    pub fn om_set_render_targets (&self, render_target_views: &[*mut ID3D11RenderTargetView], depth_stencil_view: *mut ID3D11DepthStencilView) {
        unsafe { (*self.device_context).OMSetRenderTargets(
            render_target_views.len() as UINT,
            render_target_views.as_ptr(),
            depth_stencil_view
        )};
    }

    pub fn rs_set_viewports (&self, viewports: &[D3D11_VIEWPORT]) {
        unsafe { (*self.device_context).RSSetViewports(
            viewports.len() as UINT,
            viewports.as_ptr()
        )};
    }

    pub fn clear_render_target_view (&self, render_target_view: *mut ID3D11RenderTargetView, rgba: &[FLOAT; 4]) {
        unsafe { (*self.device_context).ClearRenderTargetView(
            render_target_view,
            rgba
        )};
    }

    pub fn ia_set_input_layout (&self, input_layout: *mut ID3D11InputLayout) {
        unsafe { (*self.device_context).IASetInputLayout(input_layout) };
    }

    pub fn ia_set_primitive_topology (&self, topology: PrimitiveTopology) {
        unsafe { (*self.device_context).IASetPrimitiveTopology(topology.raw()) };
    }

    pub fn ia_set_vertex_buffers (
        &self,
        start_slot: UINT,
        vertex_buffers: &[*mut ID3D11Buffer],
        strides: &[UINT],
        offsets: &[UINT]
    ) {
        expect_eq!(vertex_buffers.len(), strides.len());
        expect_eq!(vertex_buffers.len(), offsets.len());
        let n = vertex_buffers.len().min(strides.len()).min(offsets.len());

        unsafe { (*self.device_context).IASetVertexBuffers(start_slot, n as UINT, vertex_buffers.as_ptr(), strides.as_ptr(), offsets.as_ptr()) };
    }

    pub fn vs_set_shader (&self, vertex_shader: *mut ID3D11VertexShader, class_instances: &[*mut ID3D11ClassInstance]) {
        unsafe { (*self.device_context).VSSetShader(vertex_shader, class_instances.as_ptr(), class_instances.len() as UINT) };
    }

    pub fn ps_set_shader (&self, pixel_shader: *mut ID3D11PixelShader, class_instances: &[*mut ID3D11ClassInstance]) {
        unsafe { (*self.device_context).PSSetShader(pixel_shader, class_instances.as_ptr(), class_instances.len() as UINT) };
    }

    pub fn draw (&self, vertex_count: UINT, start_vertex_location: UINT) {
        unsafe { (*self.device_context).Draw(vertex_count, start_vertex_location) };
    }
}

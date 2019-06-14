use crate::win32::*;

com_wrapper! {
    pub struct Buffer(*mut ID3D11Buffer);
    pub struct Texture2D(*mut ID3D11Texture2D);
    pub struct PixelShader(*mut ID3D11PixelShader);
    pub struct VertexShader(*mut ID3D11VertexShader);
    pub struct InputLayout(*mut ID3D11InputLayout);
    pub struct ClassInstance(*mut ID3D11ClassInstance);
    pub struct ClassLinkage(*mut ID3D11ClassLinkage);
    pub struct RenderTargetView(*mut ID3D11RenderTargetView);
    pub struct DepthStencilView(*mut ID3D11DepthStencilView);
}

mod driver_type;
mod feature_level;
mod input_element_desc;
mod primitive_topology;

mod device;
mod device_context;

pub use driver_type::*;
pub use feature_level::*;
pub use input_element_desc::*;
pub use primitive_topology::*;

pub use device::*;
pub use device_context::*;

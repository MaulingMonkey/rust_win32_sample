use crate::win32::*;
use std::ptr::{null, null_mut};

mod driver_type;
mod feature_level;
mod input_element_desc;
mod primitive_topology;

mod device;
mod device_context;
mod swap_chain;

pub use driver_type::*;
pub use feature_level::*;
pub use input_element_desc::*;
pub use primitive_topology::*;

pub use device::*;
pub use device_context::*;
pub use swap_chain::*;

pub struct DeviceAndSwapChain {
    pub feature_level:  FeatureLevel,
    pub swap_chain:     SwapChain,
    pub device:         Device,
    pub device_context: DeviceContext,
}

#[allow(dead_code)] // feature_level
impl DeviceAndSwapChain {
    pub unsafe fn create(
        adapter:            Option<&IDXGIAdapter>,
        driver_type:        DriverType,
        software:           Option<HMODULE>,
        flags:              UINT, // D3D11_CREATE_DEVICE_FLAG
        feature_levels:     Option<&[FeatureLevel]>,
        swap_chain_desc:    &DXGI_SWAP_CHAIN_DESC,
    ) -> Result<Self, HRESULT> {
        // Unsafe concerns:
        //      DXGI_SWAP_CHAIN_DESC::OutputWindow (HWND)
        //      software (HMODULE)

        let mut swap_chain      = null_mut();
        let mut device          = null_mut();
        let mut device_context  = null_mut();
        let mut feature_level   = FeatureLevel::_9_1;
        let hresult = D3D11CreateDeviceAndSwapChain(
            adapter.map_or(null_mut(), |a| a as *const _ as *mut _),
            driver_type.raw(),
            software.unwrap_or(null_mut()),
            flags,
            feature_levels.map_or(null(), |s| s.as_ptr() as *const _),
            feature_levels.map_or(0,      |s| s.len() as u32),
            D3D11_SDK_VERSION,
            swap_chain_desc,
            &mut swap_chain,
            &mut device,
            &mut feature_level.0,
            &mut device_context
        );
        if SUCCEEDED(hresult) {
            Ok(Self{
                feature_level,
                swap_chain:     SwapChain::from(swap_chain).unwrap(),
                device:         Device::from(device).unwrap(),
                device_context: DeviceContext::from(device_context).unwrap(),
            })
        } else {
            Err(hresult)
        }
    }
}

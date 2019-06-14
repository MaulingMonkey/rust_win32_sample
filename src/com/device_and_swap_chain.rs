use crate::com::*;
use crate::com::d3d11::*;
use crate::com::dxgi::*;
use crate::win32::*;
use std::ptr::{null, null_mut};

pub struct DeviceAndSwapChain {
    pub feature_level:  FeatureLevel,
    pub swap_chain:     SwapChain,
    pub device:         Device,
    pub device_context: DeviceContext,
}

#[allow(dead_code)] // feature_level
impl DeviceAndSwapChain {
    /// MSDN: [D3D11CreateDeviceAndSwapChain](https://docs.microsoft.com/en-us/windows/desktop/api/d3d11/nf-d3d11-d3d11createdeviceandswapchain)
    /// 
    /// `unsafe`:  Possible undefined behavior if `DXGI_SWAP_CHAIN_DESC::OutputWindow` is an invalid handle,
    /// or if `software` is `Some(invalid_ptr)`
    pub unsafe fn create(
        adapter:            Option<&IDXGIAdapter>,
        driver_type:        DriverType,
        software:           Option<HMODULE>,
        flags:              UINT, // D3D11_CREATE_DEVICE_FLAG
        feature_levels:     Option<&[FeatureLevel]>,
        swap_chain_desc:    &DXGI_SWAP_CHAIN_DESC,
    ) -> Result<Self, HRESULT> {
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
                swap_chain:     SwapChain::own(swap_chain).unwrap(),
                device:         Device::own(device).unwrap(),
                device_context: DeviceContext::own(device_context).unwrap(),
            })
        } else {
            Err(hresult)
        }
    }
}

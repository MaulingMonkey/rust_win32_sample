use crate::d3d11::*;
use std::ptr::{null, null_mut};

pub struct SwapChain {
    swap_chain: *const IDXGISwapChain,
}

impl Clone for SwapChain {
    fn clone (&self) -> Self {
        unsafe { (*self.swap_chain).AddRef() };
        Self { swap_chain: self.swap_chain }
    }
}

impl Drop for SwapChain {
    fn drop (&mut self) {
        unsafe { (*self.swap_chain).Release() };
    }
}

impl Into<*const IDXGISwapChain> for &SwapChain {
    fn into (self) -> *const IDXGISwapChain { self.swap_chain }
}

impl SwapChain {
    /// Take ownership of a ID3D11Device
    pub unsafe fn from (swap_chain: *const IDXGISwapChain) -> Option<Self> {
        if swap_chain == null() {
            None
        } else {
            Some(Self { swap_chain })
        }
    }

    pub fn present (&self, sync_interval: UINT, flags: UINT) -> Result<(),HRESULT> {
        let result = unsafe { (*self.swap_chain).Present(sync_interval, flags) };
        if SUCCEEDED(result) { Ok(()) }
        else { Err(result) }
    }

    pub fn get_buffer<T: Interface> (&self, buffer: UINT) -> Result<*mut T, HRESULT> {
        let mut surface = null_mut();
        let result = unsafe { (*self.swap_chain).GetBuffer(buffer, &T::uuidof(), &mut surface) };
        if SUCCEEDED(result) {
            expect_ne!(surface, null_mut());
            Ok(surface as *mut _)
        } else {
            Err(result)
        }
    }
}

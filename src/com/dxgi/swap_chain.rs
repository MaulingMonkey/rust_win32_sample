use crate::com::*;
use crate::win32::*;
use std::ptr::{null_mut};

com_wrapper! {
    pub struct SwapChain(*mut IDXGISwapChain);
}

/// MSDN: [IDXGISwapChain](https://docs.microsoft.com/en-us/windows/desktop/api/dxgi/nn-dxgi-idxgiswapchain)
impl SwapChain {
    /// MSDN: [IDXGISwapChain::Present](https://docs.microsoft.com/en-us/windows/desktop/api/dxgi/nf-dxgi-idxgiswapchain-present)
    pub fn present (&self, sync_interval: UINT, flags: UINT) -> Result<(),HRESULT> {
        let result = unsafe { self.as_ref().Present(sync_interval, flags) };
        if SUCCEEDED(result) { Ok(()) }
        else { Err(result) }
    }

    /// MSDN: [IDXGISwapChain::GetBuffer](https://docs.microsoft.com/en-us/windows/desktop/api/dxgi/nf-dxgi-idxgiswapchain-getbuffer)
    pub fn get_buffer<T: Wrapper> (&self, buffer: UINT) -> Result<T, HRESULT> {
        let mut surface = null_mut();
        let result = unsafe { self.as_ref().GetBuffer(buffer, &T::uuidof(), &mut surface) };
        if SUCCEEDED(result) {
            expect_ne!(surface, null_mut());
            Ok(unsafe { T::own(surface as *mut _) }.unwrap())
        } else {
            Err(result)
        }
    }
}

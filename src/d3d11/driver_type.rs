use crate::win32::*;

#[repr(u32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[allow(dead_code)] // Unused variants
pub enum DriverType {
    Unknown     = D3D_DRIVER_TYPE_UNKNOWN,
    Hardware    = D3D_DRIVER_TYPE_HARDWARE,
    Reference   = D3D_DRIVER_TYPE_REFERENCE,
    Null        = D3D_DRIVER_TYPE_NULL,
    Software    = D3D_DRIVER_TYPE_SOFTWARE,
    WARP        = D3D_DRIVER_TYPE_WARP,
}

impl DriverType {
    pub fn raw(self) -> D3D_DRIVER_TYPE { self as D3D_DRIVER_TYPE }
}

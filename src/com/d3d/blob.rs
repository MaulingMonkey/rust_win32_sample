use crate::win32::*;
use crate::com::Wrapper;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;

com_wrapper! {
    pub struct Blob(*mut ID3DBlob);
}

impl Blob {
    pub fn from_file<P: AsRef<Path>> (path: &P) -> Result<Blob,HRESULT> {
        let mut path : Vec<u16> = path.as_ref().as_os_str().encode_wide().collect();
        path.push(0u16);

        let mut blob = null_mut();
        let result = unsafe { D3DReadFileToBlob(path.as_ptr(), &mut blob) };
        if SUCCEEDED(result) {
            Ok(unsafe { Blob::own(blob) }.unwrap())
        } else {
            Err(result)
        }
    }

    pub fn write_file<P: AsRef<Path>> (&self, path: &P, overwrite: bool) -> Result<(),HRESULT> {
        let mut path : Vec<u16> = path.as_ref().as_os_str().encode_wide().collect();
        path.push(0u16);

        let result = unsafe { D3DWriteBlobToFile(
            self.0,
            path.as_ptr(),
            if overwrite { TRUE } else { FALSE }
        )};
        if SUCCEEDED(result) {
            Ok(())
        } else {
            Err(result)
        }
    }

    pub fn as_bytes (&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (*self.0).GetBufferPointer() as *const _,
                (*self.0).GetBufferSize()
            )
        }
    }
}

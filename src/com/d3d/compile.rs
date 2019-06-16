use crate::win32::*;
use crate::com::Wrapper;
use crate::com::d3d::Blob;
use std::ffi::CStr;
use std::path::Path;
use std::ptr::{null, null_mut};

/// This is for values like `D3D_COMPILE_STANDARD_FILE_INCLUDE` which are not real implementations of `ID3DInclude`,
/// but magic constants like `1` cast to pointers for `D3DCompile` to magically interpret on the inside.  To avoid
/// soundness problems, we need a separate type that has no safe methods that dereference the pointer.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct MagicConstantInclude(*mut ID3DInclude);
#[allow(dead_code)]
pub const COMPILE_STANDARD_FILE_INCLUDE : MagicConstantInclude = MagicConstantInclude(D3D_COMPILE_STANDARD_FILE_INCLUDE);

/// `unsafe` - `into_compile_include` is assumed to return a *valid* pointer!
pub unsafe trait IntoSafeCompileInclude                     { fn into_compile_include(&self) -> *mut ID3DInclude; }
unsafe impl IntoSafeCompileInclude for ID3DInclude          { fn into_compile_include(&self) -> *mut ID3DInclude { &*self as *const _ as *mut _ } }
unsafe impl IntoSafeCompileInclude for MagicConstantInclude { fn into_compile_include(&self) -> *mut ID3DInclude { self.0 } }

#[derive(Clone)]
pub struct ShaderAndWarnings {
    pub shader:   Blob,
    pub warnings: Option<Blob>,
}

#[derive(Clone)]
pub struct ResultAndErrors {
    pub hresult: HRESULT,
    pub errors: Option<Blob>,
}

impl std::fmt::Debug for ResultAndErrors {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "D3DCompile failed with HRESULT={:X}", self.hresult)
    }
}

/// # [D3DCompile](https://docs.microsoft.com/en-us/windows/desktop/api/d3dcompiler/nf-d3dcompiler-d3dcompile)
/// 
/// `unsafe` - `defines` contains raw pointers that must be dereferenced.
/// 
/// # Arguments
/// 
/// * `source_data` - ASCII shader source code.
/// * `source_name` - Used for error messages.
/// * `defines` - If present, must be terminated with a "NULL" `D3D_SHADER_MACRO` (e.g. one with `Name`/`Definition` being `null()`)
/// * `include` - If present, allows `#include` statements.  Consider using `Some(d3d::COMPILE_STANDARD_FILE_INCLUDE)`.
/// * `entrypoint` - Should be `None` for `fx_*` profiles, must be a valid function name (`Some("main")`?) otherwise.
/// * `target` - A valid [compiler target](https://docs.microsoft.com/en-us/windows/desktop/direct3dhlsl/specifying-compiler-targets)
pub unsafe fn compile<I: IntoSafeCompileInclude> (
    source_data:            &[u8],
    source_name:            Option<&Path>,
    defines:                Option<&[D3D_SHADER_MACRO]>,
    include:                Option<I>,
    entrypoint:             Option<&CStr>,
    target:                 &CStr,
    flags1:                 UINT,
    flags2:                 UINT,
) -> std::result::Result<ShaderAndWarnings, ResultAndErrors> {
    if let Some(defines) = defines {
        expect_ne!(defines.len(), 0);
        let last_define = defines[defines.len()-1];
        expect_eq!(null(), last_define.Name);
        expect_eq!(null(), last_define.Definition);
    }

    let mut source_name_buf = Vec::new();
    let source_name = source_name.map_or(null(), |sn| {
        source_name_buf = Vec::from(sn.as_os_str().to_string_lossy().as_bytes());
        source_name_buf.as_ptr() as *const _
    });

    let mut shader = null_mut();
    let mut errors = null_mut();
    let hresult = D3DCompile(
        source_data.as_ptr() as LPCVOID,
        source_data.len() as SIZE_T,
        source_name,
        defines.map_or(null_mut(), |d| d.as_ptr()),
        include.map_or(null_mut(), |i| i.into_compile_include()),
        entrypoint.map_or(null_mut(), |e| e.as_ptr()),
        target.as_ptr(),
        flags1,
        flags2,
        &mut shader,
        &mut errors
    );
    if SUCCEEDED(hresult) {
        Ok(ShaderAndWarnings { shader: Blob::own(shader).unwrap(), warnings: Blob::own(errors).map_or(None, |e| Some(e)) })
    } else {
        Err(ResultAndErrors { hresult, errors: Blob::own(errors).map_or(None, |e| Some(e)) })
    }
}

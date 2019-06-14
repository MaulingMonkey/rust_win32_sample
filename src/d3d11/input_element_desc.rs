use crate::win32::*;

pub trait IntoInputElements {
    fn into_input_elements (&self) -> &[D3D11_INPUT_ELEMENT_DESC];
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct InputElementDesc<'a>(pub(crate) D3D11_INPUT_ELEMENT_DESC, pub(crate) std::marker::PhantomData<&'a str>);

unsafe impl<'a> Sync for InputElementDesc<'a> {}

impl IntoInputElements for &[D3D11_INPUT_ELEMENT_DESC] {
    fn into_input_elements (&self) -> &[D3D11_INPUT_ELEMENT_DESC] {
        self
    }
}

impl<'a> IntoInputElements for &[InputElementDesc<'a>] {
    fn into_input_elements (&self) -> &[D3D11_INPUT_ELEMENT_DESC] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const _, self.len()) }
    }
}

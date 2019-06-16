pub trait Wrapper : Sized {
    type Target;

    /// Take ownership of a native COM object.  `com` is assumed to have a refcount of 1+, which is not incremented.
    unsafe fn own (com: *mut Self::Target) -> Option<Self>;

    /// Get the UUID of the native COM interface type.
    fn uuidof () -> winapi::shared::guiddef::GUID;

    /// Get a raw pointer to the native COM object type.
    fn as_ptr (&self) -> *mut Self::Target;
}

pub trait AsNativeSlice<Target> {
    fn as_native_slice (&self) -> &[Target];
}

impl<T> AsNativeSlice<T> for [T] {
    fn as_native_slice (&self) -> &[T] { self }
}

impl<T> crate::com::AsNativeSlice<*mut T> for [&T] {
    fn as_native_slice (&self) -> &[*mut T] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const _, self.len()) }
    }
}

// XXX: This has a lot of overlap with https://github.com/retep998/wio-rs/blob/master/src/com.rs
macro_rules! com_wrapper {
    ($(pub struct $wrapper:ident(*mut $target:ident);)+) => {$(
        pub struct $wrapper(*mut $target);

        impl AsRef<$target> for $wrapper {
            fn as_ref (&self) -> &$target { unsafe { &*self.0 } }
        }

        impl Clone for $wrapper {
            fn clone (&self) -> Self { unsafe { self.as_ref().AddRef() }; Self(self.0) }
        }

        impl Drop for $wrapper {
            fn drop (&mut self) { unsafe { self.as_ref().Release() }; }
        }

        impl crate::com::Wrapper for $wrapper {
            type Target = $target;

            unsafe fn own (com_object: *mut $target) -> Option<$wrapper> {
                if com_object == std::ptr::null_mut() {
                    None
                } else {
                    Some(Self(com_object))
                }
            }

            fn uuidof () -> winapi::shared::guiddef::GUID {
                $target::uuidof()
            }

            fn as_ptr (&self) -> *mut Self::Target {
                self.0
            }
        }

        impl crate::com::AsNativeSlice<*mut $target> for [Option<&$target>] {
            fn as_native_slice (&self) -> &[*mut $target] {
                // XXX: Possibly Undefined Behavior?  Option<&_> isn't explicitly #[repr(C)]...
                unsafe { std::slice::from_raw_parts(self.as_ptr() as *const _, self.len()) }
            }
        }
    )+};
}

pub mod d3d;
pub mod d3d11;
pub mod dxgi;

mod device_and_swap_chain;
pub use device_and_swap_chain::*;

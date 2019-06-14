macro_rules! wstr {
    ($str:expr) => {
        std::os::windows::ffi::OsStrExt::encode_wide(
            std::ffi::OsStr::new(concat!($str, "\0"))
        ).collect::<Vec<u16>>().as_ptr()
        // A proc_macro like wch_c (wchar 0.2.0) would let us avoid ALLOCATING for a freakin' PCWSTR literal.
        // However, that's not yet part of stable rust, so herp de derpity derp.
    };
}

macro_rules! input_layout {
    ($({ $semantic_name:expr , $semantic_index:expr , $format:expr , $input_slot:expr , $aligned_byte_offset:expr , $input_slot_class:expr , $instance_data_step_rate:expr }),+ $(,)?) => {
        [
            $(crate::d3d11::InputElementDesc(D3D11_INPUT_ELEMENT_DESC {
                SemanticName:           concat!($semantic_name, "\0").as_ptr() as *const _,
                SemanticIndex:          $semantic_index,
                Format:                 $format,
                InputSlot:              $input_slot,
                AlignedByteOffset:      $aligned_byte_offset,
                InputSlotClass:         $input_slot_class,
                InstanceDataStepRate:   $instance_data_step_rate,
            }, PhantomData)),+
        ]
    };
}

macro_rules! expect {
    ($expr:expr) => {{
        if !($expr) {
            #[allow(unused_unsafe)]
            unsafe {
                crate::win32::OutputDebugStringA(concat!(stringify!($expr), "\n... was false\n\0").as_ptr() as *const _);
                if crate::win32::IsDebuggerPresent() != 0 { crate::win32::DebugBreak(); }
                panic!(concat!("expect!(", stringify!($expr), ") failed"));
            }
        }
    }};
}

macro_rules! expect_eq {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        if left != right {
            #[allow(unused_unsafe)]
            unsafe {
                let msg = format!("expect_eq!({}, {}) failed.\nleft:  {:?}\nright: {:?}\n\0", stringify!($left), stringify!($right), &left, &right);
                crate::win32::OutputDebugStringA(msg.as_ptr() as *const _);
                if crate::win32::IsDebuggerPresent() != 0 { crate::win32::DebugBreak(); }
                panic!(msg);
            }
        }
    }};
}

macro_rules! expect_ne {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        if left == right {
            #[allow(unused_unsafe)]
            unsafe {
                let msg = format!("expect_ne!({}, {}) failed.\nleft:  {:?}\nright: {:?}\n\0", stringify!($left), stringify!($right), &left, &right);
                crate::win32::OutputDebugStringA(msg.as_ptr() as *const _);
                if crate::win32::IsDebuggerPresent() != 0 { crate::win32::DebugBreak(); }
                panic!(msg);
            }
        }
    }};
}

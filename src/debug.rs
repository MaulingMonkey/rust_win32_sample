#[inline(always)]
pub fn break_if_debugger() {
    #[cfg(windows)]
    unsafe {
        use crate::win32::*;
        // There's technically a race condition here if this gets hit at the same time a debugger (de)attaches...
        // but this is safe enough I'm not incilned to mark this fn unsafe.
        if IsDebuggerPresent() != 0 {
            DebugBreak();
        }
    }
}

pub fn init() {
    std::panic::set_hook(Box::new(|panic|{
        println!("{:?}", panic);
        break_if_debugger()
    }));
}

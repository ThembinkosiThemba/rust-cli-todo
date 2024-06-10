use std::sync::atomic::{AtomicBool, Ordering};

// TODO(#23): ctrlc module is not implemented for windows
// It's not that important right now, since ncurses crate already prevents it from working properly
// on windows anyway.
#[cfg(not(unix))]
compile_error! {"Windows is not supported right now"}

static CTRLC: AtomicBool = AtomicBool::new(false);

extern "C" fn callback(_signum: i32) {
    CTRLC.store(true, Ordering::Relaxed);
}

pub fn init() {
    unsafe {
        if libc::signal(libc::SIGINT, callback as libc::sighandler_t) == libc::SIG_ERR {
            unreachable!()
        }
    }
}

pub fn poll() -> bool {
    CTRLC.swap(false, Ordering::Relaxed)
}
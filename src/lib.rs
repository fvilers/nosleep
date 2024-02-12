#[cfg(windows)]
pub fn run() {
    use std::{thread, time::Duration};
    use winapi::um::{winbase::SetThreadExecutionState, winnt::ES_CONTINUOUS};

    let dur = Duration::from_secs(60);

    loop {
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS);
        }

        thread::sleep(dur);
    }
}

#[cfg(not(windows))]
pub fn run() {
    eprintln!("Unsupported operating system. This application only runs on Windows.")
}

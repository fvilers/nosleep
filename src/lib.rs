pub struct Options {
    timeout_in_seconds: u64,
}

const DEFAULT_TIMEOUT: u64 = 60;

impl Options {
    #[must_use]
    pub fn build(args: &[String]) -> Self {
        let timeout_in_seconds = args
            .get(1)
            .map_or(DEFAULT_TIMEOUT, |s| s.parse().unwrap_or(DEFAULT_TIMEOUT));

        Self { timeout_in_seconds }
    }
}

#[cfg(windows)]
pub fn run(options: &Options) {
    use std::{thread, time::Duration};
    use winapi::um::{winbase::SetThreadExecutionState, winnt::ES_CONTINUOUS};

    let dur = Duration::from_secs(options.timeout_in_seconds);

    loop {
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS);
        }

        thread::sleep(dur);
    }
}

#[cfg(not(windows))]
pub fn run(options: &Options) {
    eprintln!("Unsupported operating system. This application only runs on Windows.")
}

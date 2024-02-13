use std::{error::Error, mem, thread, time::Duration};

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

/// # Errors
///
/// Will return `Err` if the virtual key could not be converted to u16 or if the size of the input message could not be
/// determined.
#[cfg(windows)]
pub fn run(options: &Options) -> Result<(), Box<dyn Error>> {
    use std::ffi::c_int;
    use winapi::um::winuser::{INPUT_u, SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, VK_F15};

    let mut input = {
        unsafe {
            let mut input_u: INPUT_u = std::mem::zeroed();

            *input_u.ki_mut() = KEYBDINPUT {
                wVk: VK_F15.try_into()?,
                dwExtraInfo: 0,
                wScan: 0,
                time: 0,
                dwFlags: 0,
            };

            INPUT {
                type_: INPUT_KEYBOARD,
                u: input_u,
            }
        }
    };
    let size: c_int = mem::size_of::<INPUT>().try_into()?;
    let dur = Duration::from_secs(options.timeout_in_seconds);

    loop {
        unsafe {
            SendInput(1, &mut input, size);
        }

        thread::sleep(dur);
    }
}

#[cfg(not(windows))]
pub fn run(options: &Options) -> Result<(), Box<dyn Error>> {
    Err("Unsupported operating system. This application only runs on Windows.".into())
}

#![windows_subsystem = "windows"]

use nosleep::{run, Options};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = Options::build(&args);

    run(options);
}

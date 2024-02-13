#![windows_subsystem = "windows"]

use nosleep::{run, Options};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let options = Options::build(&args);

    if let Err(e) = run(&options) {
        eprintln!("{e}");
        process::exit(1);
    }
}

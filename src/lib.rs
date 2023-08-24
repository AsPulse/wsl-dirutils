use colored::Colorize;
use std::{process::exit, env::args};

pub fn run() {
    let args = &mut args();

    match args.nth(1) {
        Some(cmd) if cmd == "cd" => {
            unimplemented!();
        },
        Some(cmd) if cmd == "pwd" => {
            unimplemented!();
        },
        Some(cmd) => {
            eprintln!("Command {} is currently not supported.", cmd.bold());
            eprintln!("We welcomes any Issue/PR!  URL: https://github.com/AsPulse/wsl-dirutils");
            show_usage_and_exit();
        }
        None => {
            show_usage_and_exit();
        }
    }
}

pub fn show_usage_and_exit() {
    eprintln!("{}", "Usage: wsl-dirutils <cd|pwd>".red());
    exit(1);
}

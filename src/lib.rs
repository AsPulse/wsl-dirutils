mod convert;
mod lang;
mod pathutil;
mod usage;

use colored::Colorize;
use std::env::args;
use usage::{show_usage_and_exit, UsageType};

pub fn run() {
    colored::control::set_override(true);
    let args = &mut args();

    match args.nth(1) {
        Some(cmd) if cmd == "convert" => {
            convert::exec(args.next());
        }
        Some(cmd) if cmd == "pwd" => {
            unimplemented!();
        }
        Some(cmd) => {
            eprintln!("Command {} is currently not supported.", cmd.bold());
            eprintln!("We welcomes any Issue/PR!  URL: https://github.com/AsPulse/wsl-dirutils");
            show_usage_and_exit(UsageType::Root);
        }
        None => {
            show_usage_and_exit(UsageType::Root);
        }
    }
}

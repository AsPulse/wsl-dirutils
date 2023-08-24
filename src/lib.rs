mod usage;
mod cd;
mod pathutil;
mod lang;

use colored::Colorize;
use usage::{show_usage_and_exit, UsageType};
use std::env::args;


pub fn run() {
    let args = &mut args();

    match args.nth(1) {
        Some(cmd) if cmd == "cd" => {
            cd::exec(args.next());
        },
        Some(cmd) if cmd == "pwd" => {
            unimplemented!();
        },
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


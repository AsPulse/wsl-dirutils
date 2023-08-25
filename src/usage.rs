use colored::Colorize;
use std::process::exit;

pub enum UsageType {
    Root,
    CmdConvert,
}

pub fn show_usage_and_exit(usage_type: UsageType) {
    match usage_type {
        UsageType::Root => {
            eprintln!("{}", "Usage: wsl-dirutils <convert|pwd>".red());
        }
        UsageType::CmdConvert => {
            eprintln!("{}", "Usage: wsl-dirutils convert <path>".red());
        }
    }
    exit(1);
}

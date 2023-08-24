use std::process::exit;
use colored::Colorize;

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

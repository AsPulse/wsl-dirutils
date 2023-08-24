use std::process::exit;
use colored::Colorize;

pub enum UsageType {
    Root,
    CmdCd,
}

pub fn show_usage_and_exit(usage_type: UsageType) {
    match usage_type {
        UsageType::Root => {
            eprintln!("{}", "Usage: wsl-dirutils <cd|pwd>".red());
        }
        UsageType::CmdCd => {
            eprintln!("{}", "Usage: wsl-dirutils cd <path>".red());
        }
    }
    exit(1);
}

use std::process::exit;
use colored::Colorize;

pub fn run() {
    eprintln!("{}", "Usage: wsl-dirutils <cd|pwd>".red());
    exit(1);
}

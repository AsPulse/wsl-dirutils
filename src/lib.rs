use colored::Colorize;
use std::process::exit;

pub fn run() {
    eprintln!("{}", "Usage: wsl-dirutils <cd|pwd>".red());
    exit(1);
}

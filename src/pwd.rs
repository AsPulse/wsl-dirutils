use std::{process::exit, env};

use colored::Colorize;
use wslpath::wsl_to_windows;

use crate::{pathutil::is_mounted_win_path, lang::{MESSAGE_LOGO, MESSAGE_VERT}};

pub fn exec() {
    let pwd = env::current_dir().map(|path| path.to_str().map(|s| s.to_string()));

    match pwd {
        Ok(Some(pwd)) if is_mounted_win_path(&pwd) => {
            println!("{}", &pwd);
            match wsl_to_windows(&pwd) {
                Ok(windows_path) => {
                    eprintln!("{}", *MESSAGE_LOGO);
                    eprintln!("{} {} {}", *MESSAGE_VERT, &pwd, "(linux)".bright_black());
                    eprintln!("{} {} {}", *MESSAGE_VERT, &windows_path.replace("/", "\\"), "(win)".bright_black());
                }
                Err(e) => {
                    eprintln!("{}{}", *MESSAGE_LOGO, "Found Windows paths, but Unable to convert it.".red().bold());
                    eprintln!("{}{}", *MESSAGE_VERT, e.to_string().red());
                }
            }
            exit(0);
        }
        Ok(Some(pwd)) => {
            println!("{}", pwd);
            eprintln!("{}", pwd);
            exit(0);
        }
        _ => {
            eprintln!("{}", "[wsl-dirutils] Unable to get current directory.".red());
            exit(1);
        }
    }
}

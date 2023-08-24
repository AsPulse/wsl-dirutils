use colored::Colorize;
use once_cell::sync::Lazy;
use regex::Regex;
use wslpath::windows_to_wsl;

use crate::lang::{MESSAGE_LOGO, MESSAGE_VERT};
static WIN_PATH_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-z]:(\\|\/)").unwrap());

pub fn is_win_path(path: &str) -> bool {
    WIN_PATH_RE.is_match(path)
}

pub fn convert_to_wsl_with_notify(path: String) -> String {
    if !is_win_path(&path) {
        return path;
    }

    eprintln!("{}Converting... {}", *MESSAGE_LOGO, &path);

    return match windows_to_wsl(&path) {
        Ok(wsl_path) => {
            eprintln!("{}{}", *MESSAGE_VERT, &wsl_path);
            wsl_path
        }
        Err(e) =>{
            eprintln!("{}{}{}", *MESSAGE_VERT, "Unable: ".red().bold(), e.to_string().red());
            path
        }
    }
}

#[cfg(test)]
mod is_win_path_tests {
    use crate::pathutil::is_win_path;

    #[test]
    fn win_path_should_be_true() {
        assert_eq!(is_win_path(r"C:\Users\aspulse\Desktop"), true);
    }

    #[test]
    fn win_path_with_slash_should_be_true() {
        assert_eq!(is_win_path(r"C:/Users/aspulse/Desktop"), true);
    }

    #[test]
    fn linux_path_should_be_false() {
        assert_eq!(is_win_path(r"/home/aspulse"), false);
    }
}
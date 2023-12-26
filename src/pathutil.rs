use colored::Colorize;
use once_cell::sync::Lazy;
use regex::Regex;
use wslpath::windows_to_wsl;

use crate::lang::{MESSAGE_LOGO, MESSAGE_VERT};
static WIN_PATH_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-z]:(\\|\/)").unwrap());
static MOUNTED_WIN_PATH_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\/mnt\/[a-z]").unwrap());

pub fn is_win_path(path: &str) -> bool {
    WIN_PATH_RE.is_match(path)
}
pub fn is_mounted_win_path(path: &str) -> bool {
    MOUNTED_WIN_PATH_RE.is_match(path)
}

pub fn convert_to_wsl_with_notify(path: String) -> String {
    if !is_win_path(&path) {
        return path;
    }

    eprintln!(
        "{}{} {}",
        *MESSAGE_LOGO,
        "Converting...".cyan(),
        &path.bright_black()
    );

    match windows_to_wsl(&path) {
        Ok(wsl_path) if !wsl_path.is_empty() => {
            eprintln!("{}{}", *MESSAGE_VERT, &wsl_path);
            wsl_path
        }
        Ok(_) => {
            eprintln!(
                "{}{}{}",
                *MESSAGE_VERT,
                "Unable: ".red().bold(),
                "wslpath returned empty string.".red()
            );
            path
        }
        Err(e) => {
            eprintln!(
                "{}{}{}",
                *MESSAGE_VERT,
                "Unable: ".red().bold(),
                e.to_string().red()
            );
            path
        }
    }
}
#[cfg(test)]
mod is_win_path_tests {
    use crate::pathutil::is_win_path;

    #[test]
    fn win_path_should_be_true() {
        assert!(is_win_path(r"C:\Users\aspulse\Desktop"));
    }

    #[test]
    fn win_path_with_slash_should_be_true() {
        assert!(is_win_path(r"C:/Users/aspulse/Desktop"));
    }

    #[test]
    fn linux_path_should_be_false() {
        assert!(!is_win_path(r"/home/aspulse"));
    }
}
#[cfg(test)]
mod is_mounted_win_path_tests {
    use crate::pathutil::is_mounted_win_path;

    #[test]
    fn mounted_win_path_should_be_true() {
        assert!(is_mounted_win_path(r"/mnt/c/Users/aspulse/Desktop"));
    }
    #[test]
    fn in_wsl_path_should_be_false() {
        assert!(!is_mounted_win_path(r"/home/aspulse"));
    }
}

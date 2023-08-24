use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref WIN_PATH_RE: Regex = Regex::new(r"^[A-z]:(\\|\/)").unwrap();
}

pub fn is_win_path(path: &str) -> bool {
    WIN_PATH_RE.is_match(path)
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

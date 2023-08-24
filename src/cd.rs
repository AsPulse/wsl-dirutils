use std::process::exit;
use crate::{usage::{show_usage_and_exit, UsageType}, pathutil::{convert_to_wsl_with_notify, surround_if_needed}};

pub fn exec(dir: Option<String>) {
    if let Some(dir) = dir {
        println!("cd {}", surround_if_needed(convert_to_wsl_with_notify(dir)));
        exit(0);
    } else {
        show_usage_and_exit(UsageType::CmdCd);
    }
}

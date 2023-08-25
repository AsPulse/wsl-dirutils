use crate::{
    pathutil::convert_to_wsl_with_notify,
    usage::{show_usage_and_exit, UsageType},
};
use std::process::exit;

pub fn exec(dir: Option<String>) {
    if let Some(dir) = dir {
        println!("{}", convert_to_wsl_with_notify(dir));
        exit(0);
    } else {
        show_usage_and_exit(UsageType::CmdConvert);
    }
}

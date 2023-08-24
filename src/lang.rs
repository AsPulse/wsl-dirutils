use colored::Colorize;
use once_cell::sync::Lazy;

pub static MESSAGE_LOGO: Lazy<String> = Lazy::new(|| "⊘ wsl-dirutils ".bold().cyan().to_string());
pub static MESSAGE_VERT: Lazy<String> = Lazy::new(||" ⮑ ".cyan().to_string());


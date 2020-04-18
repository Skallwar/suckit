use colored::*;
use chrono::Local;

/// Time formatting string used in the call to Local::now()
/// Outputs the following:
///     <YEAR>-<MONTH>-<DAY>T<HOUR>:<MINUTES>:<SECONDS>
static TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

/// Write a message using the following format
/// <TIME_FORMAT>: [<header>] <message>
pub struct Logger {
}

impl Logger {
    pub fn info(message: String) {
        println!("{}: [{}] {}", Local::now().format(TIME_FORMAT), "INFO".blue(), message);
    }

    pub fn warn(message: String) {
        println!("{}: [{}] {}", Local::now().format(TIME_FORMAT), "WARN".yellow(), message);
    }

    pub fn error(message: String) {
        eprintln!("{}: [{}] {}", Local::now().format(TIME_FORMAT), "ERROR".red(), message);
    }
}

#[macro_export]
macro_rules! info {
    ($($arg: tt)*) => ($crate::logger::Logger::info(format!($($arg)*)));
}

#[macro_export]
macro_rules! warn {
    ($($arg: tt)*) => ($crate::logger::Logger::warn(format!($($arg)*)));
}

#[macro_export]
macro_rules! error {
    ($($arg: tt)*) => ($crate::logger::Logger::error(format!($($arg)*)));
}

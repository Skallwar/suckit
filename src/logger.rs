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

// Allow the dead code in case we don't use all the macros. Using it here means
// it doesn't affect the other modules in the crate
#[allow(dead_code)]
impl Logger {
    /// Get the current formatted timestamp
    fn get_timestamp() -> String {
        format!("{}", Local::now().format(TIME_FORMAT))
    }

    /// Display an INFO message
    pub fn info(message: String) {
        // Sadly we can't use a static format litteral so we have to retype
        // this for every function...
        println!("{}: [{}] {}", Logger::get_timestamp(), "INFO".blue(), message);
    }

    /// Display a WARNING message
    pub fn warn(message: String) {
        println!("{}: [{}] {}", Logger::get_timestamp(), "WARN".yellow(), message);
    }

    /// Display an ERROR message
    pub fn error(message: String) {
        eprintln!("{}: [{}] {}", Logger::get_timestamp(), "ERROR".red(), message);
    }
}

#[macro_export]
/// Print an info message on stdout
macro_rules! info {
    ($($arg: tt)*) => ($crate::logger::Logger::info(format!($($arg)*)));
}

#[macro_export]
/// Print a warning message on stdout
macro_rules! warn {
    ($($arg: tt)*) => ($crate::logger::Logger::warn(format!($($arg)*)));
}

#[macro_export]
/// Print an error message on stderr
macro_rules! error {
    ($($arg: tt)*) => ($crate::logger::Logger::error(format!($($arg)*)));
}

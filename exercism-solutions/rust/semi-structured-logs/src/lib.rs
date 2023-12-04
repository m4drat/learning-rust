// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        _ => panic!("Unimplemented log level!"),
    }
}
pub fn info(message: &str) -> String {
    format!("[INFO]: {message}")
}

pub fn warn(message: &str) -> String {
    format!("[WARNING]: {message}")
}

pub fn error(message: &str) -> String {
    format!("[ERROR]: {message}")
}

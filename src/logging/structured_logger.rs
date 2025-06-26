#![allow(dead_code)]

use std::{fs::File, io::Write, path::PathBuf, string::String};

// Structured Log Level Definition
pub type LogLevel = &'static str;

// Structured Log Levels
pub const LOG_LEVEL_DEBUG: LogLevel = "DEBUG";
pub const LOG_LEVEL_ERROR: LogLevel = "ERROR";
pub const LOG_LEVEL_INFO: LogLevel = "INFO";
pub const LOG_LEVEL_OFF: LogLevel = "OFF";
pub const LOG_LEVEL_TRACE: LogLevel = "TRACE";
pub const LOG_LEVEL_WARN: LogLevel = "WARN";

// Structured Log Definition
pub struct StructuredLog {
    pub current_time: String,
    pub log_level: LogLevel,
    pub log_message: String,
}

// Structured Logger
pub fn logger(structured_log: StructuredLog, log_path: PathBuf) -> () {
    let mut log_file: File = File::create(log_path).unwrap();

    log_file.write_all("".as_bytes()).unwrap();
    log_file.write_all("".as_bytes()).unwrap();
    log_file.write_all("{}".as_bytes()).unwrap();

    return ();
}

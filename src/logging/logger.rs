#![allow(dead_code)]

use std::{fs::File, io::Write, path::PathBuf, string::String, time::SystemTime};

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
    pub current_time: SystemTime,
    pub log_level: LogLevel,
    pub log_message: String,
}

// Structured Logger
pub fn structured_logger(log: StructuredLog, log_path: PathBuf) -> () {
    let mut log_file: File = File::create(log_path).unwrap();

    writeln!(log_file, "").unwrap();
    writeln!(log_file, "").unwrap();
    writeln!(log_file, "Log Level: {}", log.log_level).unwrap();
    writeln!(log_file, "{}", log.log_message).unwrap();
    writeln!(log_file, "Time: {:#?}", log.current_time).unwrap();

    return ();
}

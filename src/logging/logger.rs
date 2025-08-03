#![allow(dead_code)]

use std::{
    fs::File,
    io::{Error, StdoutLock, Write, stdout},
    path::PathBuf,
    result::{Result, Result::Ok},
    string::String,
    time::SystemTime,
};

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

// Create Log File and Write to the Log File
pub fn create_log(log: StructuredLog, log_path: PathBuf) -> Result<File, Error> {
    let mut log_file: File = File::create(log_path)?;

    writeln!(log_file, "Log Level: {}", log.log_level).unwrap();
    writeln!(log_file, "{}", log.log_message).unwrap();
    writeln!(log_file, "Time: {:#?}", log.current_time).unwrap();

    Ok(log_file)
}

// Print Log to Standard Output
pub fn print_log(log: StructuredLog) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Log Level: {}", log.log_level).unwrap();
    writeln!(standard_output, "{}", log.log_message).unwrap();
    writeln!(standard_output, "Time: {:#?}", log.current_time).unwrap();

    return ();
}

// Open Log File and Write to the Log File
pub fn write_log(log: StructuredLog, log_path: PathBuf) -> Result<File, Error> {
    let mut log_file: File = File::open(log_path)?;

    writeln!(log_file, "").unwrap();
    writeln!(log_file, "").unwrap();
    writeln!(log_file, "Log Level: {}", log.log_level).unwrap();
    writeln!(log_file, "{}", log.log_message).unwrap();
    writeln!(log_file, "Time: {:#?}", log.current_time).unwrap();

    Ok(log_file)
}

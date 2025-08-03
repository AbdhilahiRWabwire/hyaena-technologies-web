#![allow(dead_code)]

use std::{
    fs::File,
    io::{Error, StdoutLock, Write, stdout},
    option::Option,
    path::PathBuf,
    result::{Result, Result::Ok},
    string::String,
    time::SystemTime,
};

use crate::logging::logger::LogLevel;

// Application Error Definition
pub struct ApplicationError {
    pub current_time: SystemTime,
    pub error_message: String,
    pub log_level: Option<LogLevel>,
}

// Print Application Error to Standard Output
pub fn print_error(app_error: ApplicationError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", app_error.error_message).unwrap();
    writeln!(standard_output, "Time: {:#?}", app_error.current_time).unwrap();

    return ();
}

// Create Log File and Write an Error to the Log File
pub fn create_error_log(app_error: ApplicationError, log_path: PathBuf) -> Result<File, Error> {
    let mut log_file: File = File::create(log_path)?;

    writeln!(log_file, "Log Level: {:#?}", app_error.log_level).unwrap();
    writeln!(log_file, "{}", app_error.error_message).unwrap();
    writeln!(log_file, "Time: {:#?}", app_error.current_time).unwrap();

    Ok(log_file)
}

// Open Log File and Write an Error to the Log File
pub fn write_error_log(app_error: ApplicationError, log_path: PathBuf) -> Result<File, Error> {
    let mut log_file: File = File::open(log_path)?;

    writeln!(log_file, "").unwrap();
    writeln!(log_file, "").unwrap();
    writeln!(log_file, "Log Level: {:#?}", app_error.log_level).unwrap();
    writeln!(log_file, "{}", app_error.error_message).unwrap();
    writeln!(log_file, "Time: {:#?}", app_error.current_time).unwrap();

    Ok(log_file)
}

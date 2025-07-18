#![allow(dead_code)]

use std::{
    fs::File,
    io::{Error, StdoutLock, Write, stdout},
    option::Option,
    path::PathBuf,
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
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
pub fn create_error_log(app_error: ApplicationError, log_path: PathBuf) -> () {
    let log_file: Result<File, Error> = File::create(log_path);

    match log_file {
        Ok(mut file) => {
            writeln!(file, "Log Level: {:#?}", app_error.log_level).unwrap();
            writeln!(file, "{}", app_error.error_message).unwrap();
            writeln!(file, "Time: {:#?}", app_error.current_time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Creating File: {}", error);
            exit(1);
        }
    };

    return ();
}

// Open Log File and Write an Error to the Log File
pub fn write_error_log(app_error: ApplicationError, log_path: PathBuf) -> () {
    let log_file: Result<File, Error> = File::open(log_path);

    match log_file {
        Ok(mut file) => {
            writeln!(file, "").unwrap();
            writeln!(file, "").unwrap();
            writeln!(file, "Log Level: {:#?}", app_error.log_level).unwrap();
            writeln!(file, "{}", app_error.error_message).unwrap();
            writeln!(file, "Time: {:#?}", app_error.current_time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Writingrto File: {}", error);
            exit(1);
        }
    };

    return ();
}

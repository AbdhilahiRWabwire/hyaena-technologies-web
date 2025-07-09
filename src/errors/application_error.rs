#![allow(dead_code)]

use std::{
    fs::File,
    io::{Error, StdoutLock, Write, stdout},
    path::PathBuf,
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    time::SystemTime,
};

// Application Error Definition
pub struct ApplicationError {
    pub current_time: SystemTime,
    pub error_message: String,
}

// Print Application Error to Standard Output
pub fn print_error(app_error: ApplicationError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", app_error.error_message).unwrap();
    writeln!(standard_output, "Time: {:#?}", app_error.current_time).unwrap();

    return ();
}

// Open Log File and Write an Error to the Log File
pub fn log_error(app_error: ApplicationError, log_path: PathBuf) -> () {
    let log_file: Result<File, Error> = File::open(log_path);

    match log_file {
        Ok(mut file) => {
            writeln!(file, "").unwrap();
            writeln!(file, "").unwrap();
            writeln!(file, "Log Level: {}", app_error.error_message).unwrap();
            writeln!(file, "Time: {:#?}", app_error.current_time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Writingrto File: {}", error);
            exit(1);
        }
    };

    return ();
}

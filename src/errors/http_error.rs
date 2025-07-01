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
    time::SystemTime,
};

use crate::hypertext_transfer::http_status_codes::{HttpStatusCode, HttpStatusText};

// Hypertext Transfer Error Definition
pub struct HttpError {
    pub current_time: SystemTime,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
}

// Print Hypertext Transfer Error to Standard Output
pub fn print_error(http_error: HttpError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", http_error.status_code).unwrap();
    writeln!(standard_output, "{}", http_error.status_text).unwrap();
    writeln!(standard_output, "Time: {:#?}", http_error.current_time).unwrap();

    return ();
}

// Open Log File and Write an Error to the Log File
pub fn log_error(http_error: HttpError, log_path: PathBuf) -> () {
    let log_file: Result<File, Error> = File::open(log_path);

    match log_file {
        Ok(mut file) => {
            writeln!(file, "").unwrap();
            writeln!(file, "").unwrap();
            writeln!(file, "Log Level: {}", http_error.status_code).unwrap();
            writeln!(file, "{}", http_error.status_text).unwrap();
            writeln!(file, "Time: {:#?}", http_error.current_time).unwrap();
        }
        Err(error) => {
            eprintln!("Error Creating File: {}", error);
            exit(1);
        }
    };

    return ();
}

#![allow(dead_code)]

use std::{
    io::{StdoutLock, Write, stdout},
    string::String,
};

// Application Error Definition
pub struct ApplicationError {
    pub current_time: String,
    pub error_message: String,
}

// Application Error
pub fn app_error(application_error: ApplicationError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", application_error.error_message).unwrap();
    writeln!(standard_output, "Time: {}", application_error.current_time).unwrap();

    return ();
}

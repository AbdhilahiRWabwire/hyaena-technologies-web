#![allow(dead_code)]

use std::{
    io::{StdoutLock, Write, stdout},
    string::String,
    time::SystemTime,
};

// Application Error Definition
pub struct ApplicationError {
    pub current_time: SystemTime,
    pub error_message: String,
}

// Application Error
pub fn application_error(app_error: ApplicationError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", app_error.error_message).unwrap();
    writeln!(standard_output, "Time: {:#?}", app_error.current_time).unwrap();

    return ();
}

#![allow(dead_code)]

use std::{
    io::{StdoutLock, Write, stdout},
    string::String,
};

// Application Error Definition
pub struct ApplicationError<T> {
    pub current_time: String,
    pub error_message: String,
    pub error_value: T,
}

// Application Error
pub fn app_error<T>(application_error: ApplicationError<T>) {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "",).unwrap();
    writeln!(standard_output, "",).unwrap();
    writeln!(standard_output, "",).unwrap();
}

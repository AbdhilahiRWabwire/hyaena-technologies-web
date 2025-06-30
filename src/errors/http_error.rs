#![allow(dead_code)]

use std::{
    io::{StdoutLock, Write, stdout},
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

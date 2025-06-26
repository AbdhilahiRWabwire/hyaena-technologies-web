#![allow(dead_code)]

use std::{
    io::{StdoutLock, Write, stdout},
    time::SystemTime,
};

use crate::hypertext_transfer::http_status_codes::{HTTPStatusCode, HTTPStatusText};

// Hypertext Transfer Error Definition
pub struct HypertextTransferError {
    pub current_time: SystemTime,
    pub status_code: HTTPStatusCode,
    pub status_text: HTTPStatusText,
}

// Hypertext Transfer Error
pub fn hypertext_transfer_error(http_error: HypertextTransferError) -> () {
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "{}", http_error.status_code).unwrap();
    writeln!(standard_output, "{}", http_error.status_text).unwrap();
    writeln!(standard_output, "Time: {:#?}", http_error.current_time).unwrap();

    return ();
}

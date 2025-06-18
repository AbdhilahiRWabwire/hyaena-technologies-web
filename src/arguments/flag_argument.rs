#![allow(dead_code)]

use std::{process::ExitCode, string::String};

// Flag Argument Definition
pub struct FLagArgument {
    pub name: String,
    pub description: String,
    pub event: ExitCode,
}

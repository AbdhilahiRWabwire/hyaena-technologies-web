#![allow(dead_code)]

use std::{process::ExitCode, string::String};

// Command Argument Definition
pub struct CommandArgument {
    pub name: String,
    pub description: String,
    pub event: ExitCode,
}

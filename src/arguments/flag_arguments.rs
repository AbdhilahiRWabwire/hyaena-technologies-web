#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Flag Token Definition
pub type FlagToken = &'static str;

// Flag Tokens
pub const CONFIGURATION_FLAG: FlagToken = "--c";
pub const HELP_FLAG: FlagToken = "--h";
pub const SERVE_FLAG: FlagToken = "--s";
pub const VERSION_FLAG: FlagToken = "--v";

// Flags Vector
pub fn flags_vector() -> Vec<FlagToken> {
    let flags: Vec<FlagToken> =
        Vec::from([CONFIGURATION_FLAG, HELP_FLAG, SERVE_FLAG, VERSION_FLAG]);

    return flags;
}

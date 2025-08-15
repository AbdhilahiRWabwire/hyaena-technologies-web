#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Command Token Definition
pub type CommandToken = &'static str;

// Command Tokens
pub const CONFIGURATION_COMMAND: CommandToken = "configuration";
pub const EXIT_COMMAND: CommandToken = "exit";
pub const HELP_COMMAND: CommandToken = "help";
pub const SERVE_COMMAND: CommandToken = "serve";
pub const VERSION_COMMAND: CommandToken = "version";

// Commands Vector
pub fn commands_vector() -> Vec<CommandToken> {
    let commands: Vec<CommandToken> = Vec::from([
        CONFIGURATION_COMMAND,
        EXIT_COMMAND,
        HELP_COMMAND,
        SERVE_COMMAND,
        VERSION_COMMAND,
    ]);

    return commands;
}

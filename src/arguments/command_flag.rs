#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{collections::HashMap, process::ExitCode, string::String};

use crate::utility::{print_help::print_help_message, print_version::print_version_number};

// Command and Flag Argument Definition
pub struct CommandFlagArgument {
    pub name: String,
    pub description: String,
    pub event: ExitCode,
}

// Command Argument Hash Map
pub fn command_map() -> HashMap<String, CommandFlagArgument> {
    let command_arguments: HashMap<String, CommandFlagArgument> = HashMap::from([
        (
            "exit".to_string(),
            CommandFlagArgument {
                name: "exit".to_string(),
                description: "Exit Server".to_string(),
                event: ExitCode::SUCCESS,
            },
        ),
        (
            "help".to_string(),
            CommandFlagArgument {
                name: "help".to_string(),
                description: "Print List of Commands and Flags".to_string(),
                event: print_help_message(),
            },
        ),
        (
            "version".to_string(),
            CommandFlagArgument {
                name: "version".to_string(),
                description: "Print Version Number".to_string(),
                event: print_version_number(),
            },
        ),
    ]);

    return command_arguments;
}

// Flag Argument Hash Map
pub fn flag_map() -> HashMap<String, CommandFlagArgument> {
    let flag_arguments: HashMap<String, CommandFlagArgument> = HashMap::from([
        (
            "--h".to_string(),
            CommandFlagArgument {
                name: "help".to_string(),
                description: "Print List of Commands and Flags".to_string(),
                event: print_help_message(),
            },
        ),
        (
            "--v".to_string(),
            CommandFlagArgument {
                name: "version".to_string(),
                description: "Print Version Number".to_string(),
                event: print_version_number(),
            },
        ),
    ]);

    return flag_arguments;
}

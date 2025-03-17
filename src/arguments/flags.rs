use std::collections::HashMap;

use crate::utility::exit_program::successful_exit;
use crate::utility::print_help::print_help_message;
use crate::utility::print_version::print_version_number;

// Flag Argument Definition
pub struct FlagArgument {
    name: String,
    description: String,
    event: (),
}

// Flag Argument Hash Map
pub fn flag_map() -> HashMap<String, FlagArgument> {
    let mut flag_arguments: HashMap<String, FlagArgument> = HashMap::new();

    flag_arguments.insert(
        "--exit".to_string(),
        FlagArgument {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: successful_exit(),
        },
    );

    flag_arguments.insert(
        "--e".to_string(),
        FlagArgument {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: successful_exit(),
        },
    );

    flag_arguments.insert(
        "--help".to_string(),
        FlagArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );

    flag_arguments.insert(
        "--h".to_string(),
        FlagArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );

    flag_arguments.insert(
        "--version".to_string(),
        FlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );
    
    flag_arguments.insert(
        "--v".to_string(),
        FlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );

    return flag_arguments;
}

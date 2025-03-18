use std::collections::HashMap;

use super::exit_program::successful_exit;
use super::print_help::print_help_message;
use super::print_version::print_version_number;

// Command Argument Definition
#[allow(dead_code)]
pub struct CommandArgument {
    pub name: String,
    pub description: String,
    pub event: (),
}

// Command Argument Hash Map
pub fn command_map() -> HashMap<String, CommandArgument> {
    let mut command_arguments: HashMap<String, CommandArgument> = HashMap::new();

    #[allow(unreachable_code)]
    command_arguments.insert( 
        "exit".to_string(),
        CommandArgument {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: successful_exit(),
        }
    );

    #[allow(unreachable_code)]
    command_arguments.insert(
        "help".to_string(),
        CommandArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );
    
    command_arguments.insert(
        "version".to_string(),
        CommandArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );

    return command_arguments;
}
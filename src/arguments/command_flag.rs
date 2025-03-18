use std::collections::HashMap;

use super::print_help::print_help_message;
use super::print_version::print_version_number;

// Command and Flag Argument Definition
#[allow(dead_code)]
pub struct CommandFlagArgument {
    pub name: String,
    pub description: String,
    pub event: (),
}

// Command Argument Hash Map
pub fn command_map() -> HashMap<String, CommandFlagArgument> {
    let mut command_arguments: HashMap<String, CommandFlagArgument> = HashMap::new();

    #[allow(unreachable_code)]
    command_arguments.insert( 
        "exit".to_string(),
        CommandFlagArgument {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: successful_exit(),
        }
    );

    #[allow(unreachable_code)]
    command_arguments.insert(
        "help".to_string(),
        CommandFlagArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );
    
    command_arguments.insert(
        "version".to_string(),
        CommandFlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );

    return command_arguments;
}

// Flag Argument Hash Map
pub fn flag_map() -> HashMap<String, CommandFlagArgument> {
    let mut flag_arguments: HashMap<String, CommandFlagArgument> = HashMap::new();

    flag_arguments.insert(
        "--help".to_string(),
        CommandFlagArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );

    flag_arguments.insert(
        "--h".to_string(),
        CommandFlagArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );

    flag_arguments.insert(
        "--version".to_string(),
        CommandFlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );
    
    flag_arguments.insert(
        "--v".to_string(),
        CommandFlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );

    return flag_arguments;
}

// Print Help Command Output
pub fn print_help_message() -> () {
    let mut commands: HashMap<String, CommandFlagArgument> = command_map();

    let mut flags: HashMap<String, CommandFlagArgument> = flag_map();

    println!("Hyaena Technologies Web Service");
    println!("");
    println!("");
    println!("Commands:					Description:");
    println!("");

    for (command_argumen) in commands.values() {
        println!("{:#?}: {:#?}", command_argument.name, command_argument.description);
    }

    println!("");
    println!("Flags:				    Description:");
    println!("");

    for (flag_argument) in flags.values() {
        println!("{:#?}: {:#?}", flag_argument.name, flag_argument.description);
    }

    return ();
}
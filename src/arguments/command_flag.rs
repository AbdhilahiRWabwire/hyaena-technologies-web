use std::collections::HashMap;

use super::print_version::print_version_number;

use crate::utility::exit_program::successful_exit;

// Command and Flag Argument Definition
pub struct CommandFlagArgument {
    pub name: String,
    pub description: String,
}

// Command Argument Hash Map
pub fn command_map() -> HashMap<String, CommandFlagArgument> {
    let mut command_arguments: HashMap<String, CommandFlagArgument> = HashMap::new();

    command_arguments.insert( 
        "exit".to_string(),
        CommandFlagArgument {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
        }
    );

    command_arguments.insert(
        "help".to_string(),
        CommandFlagArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
        },
    );
    
    command_arguments.insert(
        "version".to_string(),
        CommandFlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
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
        },
    );

    flag_arguments.insert(
        "--h".to_string(),
        CommandFlagArgument {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
        },
    );

    flag_arguments.insert(
        "--version".to_string(),
        CommandFlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
        },
    );
    
    flag_arguments.insert(
        "--v".to_string(),
        CommandFlagArgument {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
        },
    );

    return flag_arguments;
}

// Print Help Command Output
pub fn print_help_message() -> () {
    let commands: HashMap<String, CommandFlagArgument> = command_map();
    let flags: HashMap<String, CommandFlagArgument> = flag_map();

    println!("Hyaena Technologies Web Service");
    println!("");
    println!("");
    println!("Commands:					Description:");
    println!("");

    for (command_argument, command_value) in commands {
        println!(
            "{:#?}:     {:#?}", 
            command_argument, 
            command_value.description
        );
    }

    println!("");
    println!("Flags:				    Description:");
    println!("");

    for (flag_argument, flag_value) in flags {
        println!(
            "{:#?}:     {:#?}", 
            flag_argument, 
            flag_value.description
        );
    }

    return ();
}
use std::collections::HashMap;

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
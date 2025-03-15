use std::collections::HashMap;

// Flag Definition
struct FlagArguments {
    name: String,
    description: String,
    event: fn(),
}

// Flag Map
pub fn flag_map() -> HashMap<String, FlagArguments> {
    let flag_arguments: HashMap<String, FlagArguments> = HashMap::new();

    flag_arguments.insert(
        "--exit".to_string(),
        FlagArguments {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: successful_exit(),
        },
    );

    flag_arguments.insert(
        "--e".to_string(),
        FlagArguments {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: successful_exit(),
        },
    );

    flag_arguments.insert(
        "--help".to_string(),
        FlagArguments {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );

    flag_arguments.insert(
        "--h".to_string(),
        FlagArguments {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help_message(),
        },
    );

    flag_arguments.insert(
        "--version".to_string(),
        FlagArguments {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );
    
    flag_arguments.insert(
        "--v".to_string(),
        FlagArguments {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );

    return flag_arguments;
}

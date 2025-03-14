// Command Definition
struct CommandArguments{
    name: String,
    description: String,
    event: fn(),
}

// Command Map
pub fn command_map() -> (
    String, 
    CommandArguments, 
    String, 
    CommandArguments, 
    String, 
    CommandArguments
) {
    let command_arguments: (
        String, 
        CommandArguments, 
        String, 
        CommandArguments, 
        String, 
        CommandArguments
    )  = (
        "exit".to_string(),
        CommandArguments {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: succesful_exit(),
        },
        "help".to_string(),
        CommandArguments {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help(),
        },
        "version".to_string(),
        CommandArguments {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );

    return command_arguments;
}
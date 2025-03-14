// Flag Definition
struct FlagArguments {
    name: String,
    description: String,
    event: fn(),
}

// Flag Map
pub fn flag_map() -> (
    String, 
    FlagArguments, 
    String, 
    FlagArguments, 
    String, 
    FlagArguments, 
    String, 
    FlagArguments, 
    String, 
    FlagArguments, 
    String, 
    FlagArguments
) {
    let flag_arguments: (
        String, 
        FlagArguments, 
        String, 
        FlagArguments, 
        String, 
        FlagArguments, 
        String, 
        FlagArguments, 
        String, 
        FlagArguments, 
        String, 
        FlagArguments
    ) = (
        "--exit".to_string(),
        FlagArguments {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: succesful_exit(),
        },
        "--e".to_string(),
        FlagArguments {
            name: "exit".to_string(),
            description: "Exit Server".to_string(),
            event: succesful_exit(),
        },
        "--help".to_string(),
        FlagArguments {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help(),
        },
        "--h".to_string(),
        FlagArguments {
            name: "help".to_string(),
            description: "Print List of Commands and Flags".to_string(),
            event: print_help(),
        },
        "--version".to_string(),
        FlagArguments {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
        "--v".to_string(),
        FlagArguments {
            name: "version".to_string(),
            description: "Print Version Number".to_string(),
            event: print_version_number(),
        },
    );

    return flag_arguments;
}

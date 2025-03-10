#[path = "../utility"]
pub mod utility {
    mod exit_program;
    use exit_program::succesful_exit;

    mod print_help;
    use print_help::print_help;

    mod print_version;
    use print_version::print_version_number;
}

// Flag Definition
struct Flag {
    name: &String,
    description: &String,
    event: fn()
}

// Flag Map
pub fn flag_map() -> HashMap {
    let command_arguments: HashMap = (
        "--exit", Command {
            name: "exit",
            description: "Exit Server",
            event: succesful_exit()
        },
        "--e", Command {
            name: "exit",
            description: "Exit Server",
            event: succesful_exit()
        },
        "--help", Command {
            name: "help",
            description: "Print List of Commands and Flags",
            event: print_help()
        },
        "--h", Command {
            name: "help",
            description: "Print List of Commands and Flags",
            event: print_help()
        },
        "--version", Command {
            name: "version",
            description: "Print Version Number",
            event: print_version_number()
        },
        "--v", Command {
            name: "version",
            description: "Print Version Number",
            event: print_version_number()
        }
    );

    return command_arguments;
}

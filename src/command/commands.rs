use crate::utility::exit_program::successful_exit;

use crate::utility::print_help::print_help_message;

use crate::utility::print_version::print_version_number;

// Command Definition
struct Command {
    name: &String,
    description: &String,
    event: fn(),
}

// Command Map
pub fn command_map() -> HashMap {
    let command_arguments: HashMap = (
        "exit",
        Command {
            name: "exit",
            description: "Exit Server",
            event: succesful_exit(),
        },
        "help",
        Command {
            name: "help",
            description: "Print List of Commands and Flags",
            event: print_help(),
        },
        "version",
        Command {
            name: "version",
            description: "Print Version Number",
            event: print_version_number(),
        },
    );

    return command_arguments;
}

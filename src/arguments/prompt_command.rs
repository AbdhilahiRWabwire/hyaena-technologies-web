use std::{
    io::{Stdin, StdoutLock, Write},
    process::ExitCode,
    string::String,
};

use crate::utility::{print_help::print_help_message, print_version::print_version_number};

// Command Prompt: Read, Evaluate, Print, Looop
pub fn command_prompt() -> ExitCode {
    print!("Hyaena-Technologies-Web|> ");

    let mut standard_output: StdoutLock = std::io::stdout().lock();

    standard_output.flush().unwrap();

    let standard_input: Stdin = std::io::stdin();
    let mut standard_input_buffer: String = String::new();

    while standard_input_buffer.trim() == "" {
        standard_input_buffer.clear();
        standard_input
            .read_line(&mut standard_input_buffer)
            .unwrap();
        print!("Hyaena-Technologies-Web|> ");
        standard_output.flush().unwrap();
        continue;
    }

    while standard_input_buffer.trim() != "" {
        standard_input_buffer.clear();
        standard_input
            .read_line(&mut standard_input_buffer)
            .unwrap();

        match standard_input_buffer.trim() {
            "exit" => {
                println!("Exiting Hyaena Technologies Web Service");
                return ExitCode::SUCCESS;
            }
            "help" => {
                print_help_message();
            }
            "version" => {
                print_version_number();
            }
            &_ => {
                println!("Unknown Command: {}", standard_input_buffer);
            }
        };

        print!("Hyaena-Technologies-Web|> ");
        standard_output.flush().unwrap();
        continue;
    }

    return ExitCode::SUCCESS;
}

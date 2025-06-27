use std::{
    io::{Stdin, StdoutLock, Write, stdin, stdout},
    process::ExitCode,
    string::String,
};

use crate::arguments::{print_help::print_help_message, print_version::print_version_number};

// Command Prompt: Read, Evaluate, Print, Looop
pub fn command_prompt() -> ExitCode {
    let standard_input: Stdin = stdin();
    let mut standard_input_buffer: String = String::new();
    let mut standard_output: StdoutLock = stdout().lock();

    writeln!(standard_output, "Hyaena-Technologies-Web|> ").unwrap();
    standard_output.flush().unwrap();

    while standard_input_buffer.trim() == "" {
        standard_input_buffer.clear();
        standard_input
            .read_line(&mut standard_input_buffer)
            .unwrap();
        writeln!(standard_output, "Hyaena-Technologies-Web|> ").unwrap();
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
                writeln!(standard_output, "Exiting Hyaena Technologies Web Service").unwrap();
                return ExitCode::SUCCESS;
            }
            "help" => {
                print_help_message();
            }
            "version" => {
                print_version_number();
            }
            &_ => {
                writeln!(
                    standard_output,
                    "Unknown Command: {}",
                    standard_input_buffer
                )
                .unwrap();
            }
        };

        writeln!(standard_output, "Hyaena-Technologies-Web|> ").unwrap();
        standard_output.flush().unwrap();
        continue;
    }

    return ExitCode::SUCCESS;
}

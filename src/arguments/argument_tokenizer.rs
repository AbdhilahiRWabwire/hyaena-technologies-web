use std::{
    env::args,
    io::{StdoutLock, Write, stdout},
    process::ExitCode,
    string::String,
    vec::Vec,
};

use crate::utility::{print_help::print_help_message, print_version::print_version_number};

// Command Line Argument Tokenizer
pub fn tokenize_arguments() -> ExitCode {
    let mut standard_output: StdoutLock = stdout().lock();
    let command_line_arguments: Vec<String> = args().collect();

    if command_line_arguments.len() != 2 {
        writeln!(
            standard_output,
            "Command or Flag Required but not Both: {:#?}",
            command_line_arguments
        )
        .unwrap();
        print_help_message();
        writeln!(
            standard_output,
            "Error(1) - Exiting Hyaena Technologies Web Service"
        )
        .unwrap();
        return ExitCode::FAILURE;
    } else {
        match command_line_arguments[1].trim() {
            "help" | "--h" => {
                print_help_message();
            }
            "version" | "--v" => {
                print_version_number();
            }
            &_ => {
                writeln!(
                    standard_output,
                    "Uknown Command or Flag: {}",
                    command_line_arguments[1]
                )
                .unwrap();
                print_help_message();
                writeln!(
                    standard_output,
                    "Error(1) - Exiting Hyaena Technologies Web Service"
                )
                .unwrap();
                return ExitCode::FAILURE;
            }
        };
    };

    return ExitCode::SUCCESS;
}

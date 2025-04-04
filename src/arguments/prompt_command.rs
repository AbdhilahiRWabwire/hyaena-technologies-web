use std::{
    io::{Stdin, Stdout, Write},
    process::ExitCode,
    string::String
};

use crate::utility::{
    print_help::print_help_message,
    print_version::print_version_number
};

// Command Prompt: Read, Evaluate, Print, Looop
pub fn command_prompt() -> ExitCode {
    print!("Hyaena-Technologies-Web|> ");

    let mut command_output_buffer: Stdout = std::io::stdout();

    command_output_buffer.flush().unwrap();

    let command_input: Stdin = std::io::stdin();
    let mut command_input_buffer: String = String::new();
    
    while command_input_buffer.trim() == "" {
        command_input_buffer.clear();
        command_input.read_line(&mut command_input_buffer).unwrap();
        print!("Hyaena-Technologies-Web|> ");
        command_output_buffer.flush().unwrap();
        continue;
    };

    while command_input_buffer.trim() != "" {
        command_input_buffer.clear();
        command_input.read_line(&mut command_input_buffer).unwrap();

        match command_input_buffer.trim() {
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
                println!("Unknown Command: {}", command_input_buffer);
            }
        };

        print!("Hyaena-Technologies-Web|> ");
        command_output_buffer.flush().unwrap();
        continue;
    };
    
    return ExitCode::SUCCESS;
}
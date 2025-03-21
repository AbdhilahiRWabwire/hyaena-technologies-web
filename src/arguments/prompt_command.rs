use std::io::{Stdin, Stdout, Write};

use crate::utility::{
    exit_program::{error_exit, successful_exit}, 
    print_help::print_help_message,
    print_version::print_version_number
};

// Command Prompt: Read, Evaluate, Print, Looop
pub fn command_prompt() -> () {
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
    }

    while command_input_buffer.trim() != "" {
        command_input_buffer.clear();
        command_input.read_line(&mut command_input_buffer).unwrap();

        match command_input_buffer.trim() {
            "exit" => successful_exit(),
            "help" => print_help_message(),
            "version" => print_version_number(),
            &_ => error_exit()
        }

        print!("Hyaena-Technologies-Web|> ");
        command_output_buffer.flush().unwrap();
        continue;
    }
    
    return ();
}
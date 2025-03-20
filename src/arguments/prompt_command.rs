use std::collections::HashMap;
use std::io::{Stdin, Stdout, Write};

use super::command_flag::{
    CommandFlagArgument, 
    command_map,
};

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
    let commands: HashMap<String, CommandFlagArgument> = command_map();
    
    while command_input_buffer.len() == 0 {
        command_input_buffer.clear();
        command_input.read_line(&mut command_input_buffer).unwrap();
        print!("Hyaena-Technologies-Web|> ");
        command_output_buffer.flush().unwrap();
        continue;
    }

    while command_input_buffer.len() != 0 {
        command_input_buffer.clear();
        command_input.read_line(&mut command_input_buffer).unwrap();

        match command_input_buffer.trim() {
            "exit" => successful_exit(),
            "help" => print_help_message(),
            "version" => print_version_number(),
            "--help" => print_help_message(),
            "--h" => print_help_message(),
            "--version" => print_version_number(),
            "--v" => print_version_number(),
            &_ => error_exit()
        }

        print!("Hyaena-Technologies-Web|> ");
        command_output_buffer.flush().unwrap();
        continue;
    }

    println!("Exiting Hyaena Technologies Web Service");
    return ();
}
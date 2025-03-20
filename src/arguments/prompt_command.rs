use std::collections::HashMap;
use std::io::{Stdin, Stdout, Write};

use super::command_flag::{
    CommandFlagArgument,
    command_map,
    print_help_message
};

// Command Prompt: Read, Evaluate, Print, Looop
pub fn command_prompt() -> () {
    print!("Hyaena-Technologies-Web|> ");

    let mut command_output_buffer: Stdout = std::io::stdout();

    command_output_buffer.flush().unwrap();

    let command_input: Stdin = std::io::stdin();
    let mut command_input_buffer: String = String::new();
    let commands: HashMap<String, CommandFlagArgument> = command_map();

    while command_input_buffer.len() != 0 {
        for command_argument in commands.values() {
            if command_input_buffer.trim() != command_argument.name.trim() {
                println!("Unkown Command: {:#?}", command_input_buffer.trim());
                print_help_message();
                continue;
            } else {
                while command_input_buffer.trim() == command_argument.name.trim() {
                    command_input_buffer.clear();
                    command_input.read_line(&mut command_input_buffer).unwrap();
                    command_argument.event;
                    print!("Hyaena-Technologies-Web|> ");
                    command_output_buffer.flush().unwrap();
                }
            }
            
        }
    }

    while command_input_buffer.len() == 0 {
        print!("Hyaena-Technologies-Web|> ");
        command_output_buffer.flush().unwrap();
        continue;
    }

    while command_input_buffer.trim() != "quit" {
        command_input_buffer.clear();
        command_input.read_line(&mut command_input_buffer).unwrap();
        println!("You Wrote: {:#?}", command_input_buffer.trim());
        print!("Hyaena-Technologies-Web|> ");
        command_output_buffer.flush().unwrap();
    }

    println!("Exiting Hyaena Technologies Web Service");
    return ();
}
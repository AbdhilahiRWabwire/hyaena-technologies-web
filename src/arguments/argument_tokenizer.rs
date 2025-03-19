use std::collections::HashMap;
use std::env::Args;
use std::env::args;
use std::io::BufRead;

use super::command_flag::CommandFlagArgument;
use super::command_flag::command_map;
use super::command_flag::flag_map;
use super::command_flag::print_help_message;

// Command Line Argument Tokenizer
pub fn tokenize_arguments() {
    let mut command_line_arguments: Args = args();
    
    let mut buffered_reader: BufRead<Args> = BufRead::new(command_line_arguments);

    let mut command_input_buffer: String = String::new();

    buffered_reader.read_line(&mut command_input_buffer)?;

    let mut commands: HashMap<String, CommandFlagArgument> = command_map();

    let mut flags: HashMap<String, CommandFlagArgument> = flag_map();
    
    if command_line_arguments.count() != 2 {
        println!("Command or Flag Required but not Both: {:#?}", command_line_arguments);

        print_help_message();
    } else {
        for (command_argument) in commands.keys() {
            if command_line_arguments[2] != command_argument.starts_with("--") && command_line_arguments != command_argument {
                println!("Unkown Command: {:#?}", command_line_arguments);

                print_help_message();
            }
        }

        for (flag_argument) in flags.keys() {
            if command_line_arguments[2] = flag_argument.starts_with("--") && command_line_arguments != flag_argument {
                println!("Uknown Flag: {:#?}", command_line_arguments);

                print_help_message();
            }
        }
    }
}

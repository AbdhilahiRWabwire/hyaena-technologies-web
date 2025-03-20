use std::collections::HashMap;
use std::env::args;

use super::command_flag::{
    CommandFlagArgument, 
    command_map, 
    flag_map, 
    print_help_message
};

use crate::utility::exit_program::error_exit;

// Command Line Argument Tokenizer
#[allow(dead_code)]
pub fn tokenize_arguments() {
    let command_line_arguments: Vec<String> = args().collect();
    let commands: HashMap<String, CommandFlagArgument> = command_map();
    let flags: HashMap<String, CommandFlagArgument> = flag_map();
    
    if command_line_arguments.len() != 2 {
        println!("Command or Flag Required but not Both: {:#?}", command_line_arguments);
        print_help_message();
    } else {
        for command_argument in commands.keys() {
            if command_line_arguments.get(1).unwrap() != command_argument.trim() {
                println!("Unkown Command: {:#?}", command_line_arguments);
                print_help_message();
                error_exit();
            }
        }

        for flag_argument in flags.keys() {
            if command_line_arguments.get(1).unwrap() != flag_argument.trim() {
                println!("Uknown Flag: {:#?}", command_line_arguments);
                print_help_message();
                error_exit();
            }
        }
    }
}

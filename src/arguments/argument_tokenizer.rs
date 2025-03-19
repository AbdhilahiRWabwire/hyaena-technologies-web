use std::collections::HashMap;
use std::env::Args;
use std::env::args;

use futures::io::BufReader;

use super::command_flag::CommandFlagArgument;
use super::command_flag::command_map;
use super::command_flag::flag_map;

// Command Line Argument Tokenizer
pub fn tokenize_arguments() {
    let mut command_line_arguments: Args = args();

    let mut buffered_reader: BufReader<Args> = BufReader::new(command_line_arguments);

    #[allow(unused_variables)]
    let mut commands: HashMap<String, CommandFlagArgument> = command_map();

    #[allow(unused_variables)]
    let mut flags: HashMap<String, CommandFlagArgument> = flag_map();
    
    if command_line_arguments.by_ref().count() < 2 {
        println!("Command or Flag Required: {:#?}", command_line_arguments);
    }

    for (command_argument) in commands.keys() {
        if command_line_arguments.by_ref() != command_argument.chars() {
            println!("Command or Flag Required: {:#?}", command_line_arguments);
        }
    }

    for (flag_argument) in flags.keys() {
        if command_line_arguments.by_ref() != flag_argument.chars() {
            println!("Command or Flag Required: {:#?}", command_line_arguments);
        }
    }
    
}

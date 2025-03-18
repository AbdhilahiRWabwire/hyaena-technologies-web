use std::collections::HashMap;
use std::env::Args;
use std::env::args;

use super::command_flag::CommandFlagArgument;
use super::command_flag::command_map;
use super::command_flag::flag_map;

// Command Line Argument Tokenizer
pub fn tokenize_arguments() {
    let command_line_arguments: Args = args();

    #[allow(unused_variables)]
    let mut commands: HashMap<String, CommandFlagArgument> = command_map();

    #[allow(unused_variables)]
    let mut flags: HashMap<String, CommandFlagArgument> = flag_map();
    
    if command_line_arguments.count() < 2 {
        println!("Command or Flag Required");
    }
}

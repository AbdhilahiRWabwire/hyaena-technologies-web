use std::collections::HashMap;
use std::env::Args;
use std::env::args;

use super::commands::CommandArgument;
use super::commands::command_map;
use super::flags::FlagArgument;
use super::flags::flag_map;

// Command Line Argument Tokenizer
pub fn tokenize_arguments() {
    let command_line_arguments: Args = args();

    #[allow(unused_variables)]
    let commands: HashMap<String, CommandArgument> = command_map();

    #[allow(unused_variables)]
    let flags: HashMap<String, FlagArgument> = flag_map();

    if command_line_arguments.count() < 2 {
        println!("Command or Flag Required");
    }
}

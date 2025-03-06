use std::env::args;
use std::fmt;

#[path = "./"]
mod command {
    mod commands;
    use commands::command_map;
    
    mod flags;
    use flags::flag_map;
}

// Command Line Argument Tokenizer
pub fn tokenize_arguments() {
    let command_line_arguments: Args = args();

    if command_line_arguments < 2 {
        format!("Command or Flag Required");
    }
}


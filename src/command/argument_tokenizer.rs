use std::env::args;

#[path = "./"]
mod command {
    mod commands;
    use commands::command_map;
    
    mod flags;
    use flags::flag_map;
}


pub fn tokenize_arguments() {
    let command_line_arguments: Args = args();

    if command_line_arguments < 2 {
        println!("Command or Flag Required");
    }
}


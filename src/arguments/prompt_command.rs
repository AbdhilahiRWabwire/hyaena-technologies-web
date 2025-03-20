use std::io::{Stdin, Stdout, Write};

// Command Prompt: Read, Evaluate, Print, Looop
pub fn command_prompt() -> () {
    print!("Hyaena-Technologies-Web|> ");

    let mut command_output_buffer: Stdout = std::io::stdout();

    command_output_buffer.flush().unwrap();

    let command_input: Stdin = std::io::stdin();
    let mut command_input_buffer: String = String::new();
    
    while command_input_buffer.len() == 0 {
        command_input_buffer.clear();
        command_input.read_line(&mut command_input_buffer).unwrap();
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
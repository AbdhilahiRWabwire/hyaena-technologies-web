mod arguments;
use arguments::prompt_command::command_prompt;

mod networking;
mod yaml;
mod utility;

// Main Entry Point
fn main() -> () {
    command_prompt();
    return ();
}

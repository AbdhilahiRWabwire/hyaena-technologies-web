mod arguments;
mod networking;
mod yaml;
use arguments::command_prompt::prompt_command;

// Main Entry Point
fn main() -> () {
    prompt_command();

    return ();
}

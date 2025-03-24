mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod networking;
use networking::serve_application::start_service;

mod yaml;
mod utility;

// Main Entry Point
fn main() -> () {
    start_service();
    return ();
}

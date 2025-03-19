mod arguments;
use arguments::argument_tokenizer::tokenize_arguments;

mod networking;
mod yaml;
mod utility;

// Main Entry Point
fn main() -> () {
   tokenize_arguments();

    return ();
}

mod arguments;
mod networking;
mod utility;
mod yaml;
use crate::arguments::argument_tokenizer::tokenize_arguments;

// Main Entry Point
fn main() -> () {
    println!("Hyaena Technologies");

    tokenize_arguments();

    return ();
}

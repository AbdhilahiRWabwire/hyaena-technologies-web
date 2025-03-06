use std::fmt;

#[path = "./command"]
mod command {
    mod argument_tokenizer;
    use argument_tokenizer::tokenize_arguments;
}

// Main Entry Point
fn main() {
    format!("Hyaena Technologies");
}

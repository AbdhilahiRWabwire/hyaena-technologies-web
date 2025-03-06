#[path = "./command"]
mod command {
    mod argument_tokenizer;
    use argument_tokenizer::tokenize_arguments;
}


fn main() {
    println!("Hyaena Technologies");
}

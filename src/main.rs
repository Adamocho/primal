use std::{env, fs, fmt::write};
use primal::lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read a file");

    let mut lexer = lexer::Lexer::new(0);
    lexer.tokenize(file_contents);
}

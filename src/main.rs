use std::{env, fs};
use primal::{emitter, lexer, parser};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read a file");

    let tokens = lexer::Lexer::tokenize(file_contents);

    let mut parser = parser::Parser::new(tokens);
    let parsed = parser.parse();

    let mut emitter = emitter::Emitter::new(parsed);
    let lines = emitter.emit();

    fs::write("./primal-runner/src/main.rs", lines.join("\n")).expect("Writing lines to output rust file");
}

use std::{env, fs};
use primal::{emitter, lexer, parser, optimiser};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read a file");

    let tokens = lexer::Lexer::tokenize(file_contents);
    println!("Tokens are ready!");

    let mut parser = parser::Parser::new(tokens);
    // an Abstract Syntax Tree
    let ast = parser.parse();
    println!("Abstract Syntax Tree has been constructed!");

    // dbg!(&ast);
    // let mut optimiser = optimiser::Optimiser::new();
    // let ast = optimiser.optimise(ast);

    let mut emitter = emitter::Emitter::new(ast);
    let lines = emitter.emit();

    fs::write("./primal-runner/src/main.rs", lines.join("\n")).expect("Writing lines to output rust file");
    println!("Wrote to file");
}


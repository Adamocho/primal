use core::panic;
use std::{env, fs, io::ErrorKind};
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

    let directory = fs::create_dir("./src/bin");
    if directory.is_err_and(|err| err.kind() != ErrorKind::AlreadyExists) {
        panic!("Could not create '/src/bin' directory");
    }

    fs::write("./src/bin/program.rs", lines.join("\n")).expect("Writing lines to output rust file");

    // run program
    include!("bin/program.rs");
}

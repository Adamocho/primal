use std::process::exit;

use crate::lexer::Token;

type Tree = Vec<Statement>;
type Statement = Vec<Token>;

struct Parser {
    tree: Tree,
}

impl Parser {
    fn parse(tokens: Vec<Token>) -> Tree {
         

        // for now
        vec![]
    }

    // recognize grammar-tree statement
    fn parse_token(token: Token) {

    }

    // if it's worth looking for the token in the grammar-tree
    fn find_token(token: Token) -> (bool, u32) {

    }

    fn abort(message: String) {
        println!("Syntax error: {}", message);
        exit(1);
    }
}

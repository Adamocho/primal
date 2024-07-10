use core::panic;
use std::process::exit;

use crate::lexer::Token;

// type Tree = Vec<Statement>;
// type Statement = Vec<Token>;

struct Parser {
    // tree: Tree,
}

impl Parser {
    fn parse(tokens: Vec<Token>) -> Vec<Token> {
         
        let mut tokens = tokens.iter();
        loop {
            if let Some(token) = tokens.next() {
                Self::recognize_token(token);
            } else {
                break;
            }
        }

        // for now
        vec![]
    }

    // recognize grammar-tree statement
    fn recognize_token(token: &Token) {
        // recognize start of statement
        match token {
            Token::Print => {
                // check next 
            },
            Token::Let => {
            },
            Token::If => {
            },
            Token::While => {
            },
            Token::Input => {
            },
            Token::Goto => {
            },
            Token::Newline => {
            },
             
            _ => {
                panic!("Syntax error: Token not recognized: {:?}", token);
            }
        }
    }

    // if it's worth looking for the token in the grammar-tree
    fn find_token(token: Token) -> (bool, u32) {

    }

    fn abort(message: String) {
        println!("Syntax error: {}", message);
        exit(1);
    }
}

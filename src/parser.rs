use core::panic;
use std::{any::Any, process::exit};

use crate::lexer::Token;

// THINGS:
// bubble up the syntax-error messages if possible
// dont know the type of `tree` yet

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    start: usize,
    counter: usize,
    program: Vec<Token>, // dont know
    current: Option<Token>,
    next: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            start: 0,
            counter: 0,
            program: vec![],
            current: None,
            next: None,
        }
    }

    pub fn parse(&mut self) {
        // initialize
        self.next_token();
        self.next_token();

        // while self.next.is_some() && self.counter <= self.tokens.len() {
        //     self.recognize_statement();
        // }

        self.recognize_statement();

        println!("Parsing done!");
    }

    // recognize grammar-tree statement
    fn recognize_statement(&mut self) {
        match self.current {
            // "PRINT" string nl
            Some(Token::Print) => {
                println!("Print-statement");

                self.next_token();
                self.match_token(Token::String("".to_string()));

                self.newline();

                dbg!(&self);
                todo!("IMPLEMENT THIS FURTHER");
            },
            Some(Token::Let) => {
                println!("Let-statement");
            },
            Some(Token::If) => {
                println!("If-statement");
            },
            Some(Token::While) => {
                println!("While-statement");
            },
            Some(Token::Input) => {
                println!("Input-statement");
            },
            Some(Token::Goto) => {
                println!("Goto-statement");
            },
            Some(Token::Newline) => {
                println!("Newline");
            },
            _ => {
                println!("Syntax error: Token not recognized: {:#?}", self.current);
                exit(1);
            }
        }
        println!("Passed")
    }

    // match current
    fn match_token(&mut self, expected: Token) {
        match expected {
            Token::Bool(_) | Token::String(_) | Token::Number(_) | Token::Identifier(_) => {
                println!("Not checked until you find a better solution...");
            }
            _ => {
                if Some(&expected) != self.current.as_ref() {
                    Self::abort(format!("expected {:#?}, got {:#?}", expected, self.current));
                }
            }
        }

        self.next_token()
    }

    fn next_token(&mut self) {
        self.current = self.next.clone();
        self.next = self.tokens.get(self.counter).cloned();
        self.counter += 1;
    }

    // nl ::= '\n'+
    fn newline(&mut self) {
        // require one
        self.match_token(Token::Newline);

        // remove unnecessary newlines
        while self.current == Some(Token::Newline) {
            self.next_token();
        }
    }

    fn abort(message: String) {
        println!("Syntax error: {}", message);
        exit(1);
    }
}


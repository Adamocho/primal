use core::panic;

use crate::lexer::Token;
use crate::lexer::{
    BOOL_ID, 
    STRING_ID, 
    NUMBER_ID, 
    IDENTIFIER_ID};

#[derive(Debug)]
pub enum Statment {
    Print { value: Token },
    Let { identifier: Token, value: Token },
    If { comparisons: Vec<Token>, statements: Vec<Statment> },
    While { comparisons: Vec<Token>, statements: Vec<Statment> },
    Input { string: Token, identifier: Token },
    Goto { identifier: Token },
    Empty,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    counter: usize,
    current: Option<Token>,
    next: Option<Token>,
}

const PLACEHOLDER: String = String::new();

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            counter: 0,
            current: None,
            next: None,
        }
    }

    pub fn parse(&mut self) {
        // initialize
        self.next_token();
        self.next_token();

        while self.next.is_some() && self.counter <= self.tokens.len() {
            println!("{:?}", self.recognize_statement());
        }

        println!("Parsing done!");
    }

    // recognize grammar-tree statement
    fn recognize_statement(&mut self) -> Statment {
        match self.current {
            // "PRINT" value nl
            Some(Token::Print) => {
                println!("Print-statement");
                self.next_token();

                self.value();

                let value = self.current.clone().unwrap();

                self.newline();

                Statment::Print { value }
            },
            // "LET" identifier "=" value nl
            Some(Token::Let) => {
                println!("Let-statement");
                self.next_token();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));

                let identifier = self.current.clone().unwrap();

                self.match_token(Token::Assign);

                self.value();

                let value = self.current.clone().unwrap();

                self.newline();
                
                Statment::Let { identifier, value }
            },
            // "IF" comparisons "THEN" nl {statement} nl "ENDIF" nl
            Some(Token::If) => {
                println!("If-statement");
                self.next_token();

                let start = self.counter;

                self.comparisons();
                self.match_token(Token::Then);

                let comparisons = self.tokens.get((start - 2)..(self.counter - 2)).unwrap().to_vec();

                self.match_token(Token::Newline);

                let mut statements: Vec<Statment> = Vec::new();
                while self.current != Some(Token::Endif) {
                    statements.push(self.recognize_statement());
                }

                self.match_token(Token::Endif);
                self.newline();

                Statment::If { comparisons, statements }
            },
            // "WHILE" comparisons nl "DO" nl {statement} nl "ENDWHILE" nl
            Some(Token::While) => {
                println!("While-statement");
                self.next_token();

                let start = self.counter;

                self.comparisons();
                self.newline();

                let comparisons = self.tokens.get((start - 2)..(self.counter - 2)).unwrap().to_vec();

                self.match_token(Token::Do);

                let mut statements: Vec<Statment> = Vec::new();
                self.match_token(Token::Newline);
                while self.current != Some(Token::Endwhile) {
                    statements.push(self.recognize_statement());
                }

                self.match_token(Token::Endwhile);
                self.newline();

                Statment::While { comparisons, statements }
            },
            // "INPUT" string identifier nl
            Some(Token::Input) => {
                println!("Input-statement");
                self.next_token();

                self.match_token(Token::String(PLACEHOLDER, STRING_ID));
                let string = self.current.clone().unwrap();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));
                let identifier = self.current.clone().unwrap();

                self.newline();

                Statment::Input { string, identifier }
            },
            // "GOTO" identifier nl
            Some(Token::Goto) => {
                println!("Goto-statement");
                self.next_token();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));
                let identifier = self.current.clone().unwrap();

                self.newline();

                Statment::Goto { identifier }
            },
            // nl ::= '\n'+
            Some(Token::Newline) => {
                println!("Newline");
                self.next_token();

                // clear newlines till next different token
                if self.current == Some(Token::Newline) {
                    self.newline();
                }

                Statment::Empty
            },
            _ => {
                panic!("Syntax error: Token not recognized: {:#?}", self.current);
            }
        }
    }

    // comparisons ::= comparison {("AND" | "OR") comparison}
    fn comparisons(&mut self) {
        println!("comparisons - plural");

        self.comparison();

        while self.current == Some(Token::And) || self.current == Some(Token::Or) {
            self.next_token();

            self.comparison();
        } 
    }

    // comparison ::= (expression equals expression) | ("true" | "false")
    fn comparison(&mut self) {
        println!("Checking a comparison");
        
        if let Some(Token::Bool(_, _)) | Some(Token::Identifier(_, _)) = self.current {
            self.next_token();
            return;
        }

        self.expression();

        self.equals();

        self.expression();
    }

    // value ::= identifier | string | number | bool
    fn value(&mut self) {
        println!("Value");

        match self.current {
            Some(Token::Identifier(_, id))
            | Some(Token::String(_, id))
            | Some(Token::Number(_, id))
            | Some(Token::Bool(_, id)) => {
                if id == IDENTIFIER_ID
                || id == STRING_ID
                || id == NUMBER_ID
                || id == BOOL_ID { 
                    self.next_token(); return;
                }
            },
            _ => {}
        }

        Self::abort(
            format!(
                "Expected one of: {:#?}, {:#?}, {:#?}, or {:#?}. Got: {:#?}",
                Token::Identifier(PLACEHOLDER, IDENTIFIER_ID),
                Token::String(PLACEHOLDER, STRING_ID),
                Token::Number(0, NUMBER_ID),
                Token::Bool(false, BOOL_ID),
                self.current
            )
        );
        
    }

    // expression ::= term {("+" | "-") term}
    fn expression(&mut self) {
        println!("Evaluating expression");

        self.term();

         while self.current == Some(Token::Plus) || self.current == Some(Token::Minus) {
            self.next_token();

            self.term();
        }

    }

    // term ::= unary {("*" | "/" | "%") unary}
    fn term(&mut self) {
        println!("Term");

        self.unary();

         while self.current == Some(Token::Times)
        || self.current == Some(Token::Divide)
        || self.current == Some(Token::Modulo) {
            self.next_token();

            self.unary();
        }
    }

    // unary ::= ["+" | "-"] primary
    fn unary(&mut self) {
        println!("Unary");

        if self.current == Some(Token::Plus) || self.current == Some(Token::Minus) {
            self.next_token();
        }

        self.primary();
    }

    // primary ::= identifier | number
    fn primary(&mut self) {
        println!("Primary");

        match self.current {
            Some(Token::Identifier(_, id)) | Some(Token::Number(_, id)) => {
                if id == IDENTIFIER_ID || id == NUMBER_ID { self.next_token(); return; }
            },
            _ => {}
        }

        Self::abort(
            format!(
                "expected {:#?} OR {:#?}, got {:#?}",
                Token::Identifier(PLACEHOLDER, IDENTIFIER_ID),
                Token::Number(0, NUMBER_ID),
                self.current
            )
        );
    }

    // equals ::= ("==" | "!=" | "<=" | ">=" | ">" | "<")
    fn equals(&mut self) {
        match self.current {
            Some(Token::Equals) | Some(Token::NotEquals) |
            Some(Token::MoreThan) | Some(Token::MoreThanEquals) |
            Some(Token::LessThan) | Some(Token::LessThanEquals) => {
                self.next_token();
            }
            _ => {
                Self::abort(
                    format!(
                        "expected one of: {:#?}, {:#?}, {:#?}, {:#?}, {:#?} or {:#?}, got {:#?} instead",
                        Some(Token::Equals),
                        Some(Token::NotEquals),
                        Some(Token::MoreThan),
                        Some(Token::MoreThanEquals),
                        Some(Token::LessThan),
                        Some(Token::LessThanEquals),
                        self.current
                ));
            }
        }
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

    // match current
    fn match_token(&mut self, expected: Token) {
        match expected {
            Token::Bool(_, id) | Token::String(_, id) | Token::Number(_, id) | Token::Identifier(_, id) => {
                self.match_value_token(expected, id);
            }

            _ => {
                if Some(&expected) != self.current.as_ref() {
                    Self::abort(format!("expected {:#?}, got {:#?}", expected, self.current));
                }
            }
        }
        self.next_token();
    }

    // match special cases of tokens - Bool, String, Number, Identifier
    fn match_value_token(&mut self, expected: Token, id: u8) {
        match self.current {
            Some(Token::Bool(_, current_id)) | Some(Token::String(_, current_id)) |
            Some(Token::Number(_, current_id)) | Some(Token::Identifier(_, current_id)) => {
                if id == current_id { return; }
            }
            _ => {}
        }
        Self::abort(format!("expected {:#?}, got {:#?}", expected, self.current));
    }

    fn next_token(&mut self) {
        self.current.clone_from(&self.next);
        self.next = self.tokens.get(self.counter).cloned();
        self.counter += 1;
    }

    fn abort(message: String) {
        panic!("Syntax error: {}", message);
    }
}


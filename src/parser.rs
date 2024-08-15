use core::panic;

use crate::lexer::Token;
use crate::lexer::{
    BOOL_ID, 
    STRING_ID, 
    NUMBER_ID, 
    IDENTIFIER_ID};

#[derive(Debug)]
pub enum Statement {
    Print { value: Token },
    Let { identifier: Token, expression: Vec<Token> },
    If { comparisons: Vec<Token>, statements: Vec<Statement> },
    While { comparisons: Vec<Token>, statements: Vec<Statement> },
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
    previous: Option<Token>,
}

const PLACEHOLDER: String = String::new();

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            counter: 0,
            current: None,
            next: None,
            previous: None,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        // initialize current, next, counter
        self.next_token();
        self.next_token();

        let mut statements: Vec<Statement> = vec![];

        while self.next.is_some() && self.counter <= self.tokens.len() {
            // println!("{:?}", self.recognize_statement());
            statements.push(self.recognize_statement());
        }
        statements
    }

    // recognize grammar-tree statement
    fn recognize_statement(&mut self) -> Statement {
        match self.current {
            // "PRINT" value nl
            Some(Token::Print) => {
                self.next_token();

                self.value();

                let value = self.previous.clone().unwrap();

                self.newline();

                Statement::Print { value }
            },
            // "LET" identifier "=" value nl
            Some(Token::Let) => {
                self.next_token();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));

                let identifier = self.previous.clone().unwrap();

                self.match_token(Token::Assign);

                let expression;
                let start;

                if self.next == Some(Token::Newline) {
                    self.value();
                    expression = vec![self.previous.clone().unwrap()];
                } else {
                    start = self.counter;
                    self.expression();
                    expression = self.get_range_of_tokens(start, self.counter);
                }

                self.newline();
                
                Statement::Let { identifier, expression }
            },
            // "IF" comparisons "THEN" nl {statement} nl "ENDIF" nl
            Some(Token::If) => {
                self.next_token();

                let start = self.counter;

                self.comparisons();
                self.match_token(Token::Then);

                let comparisons = self.get_range_of_tokens(start, self.counter - 1);

                self.match_token(Token::Newline);

                let mut statements: Vec<Statement> = Vec::new();
                while self.current != Some(Token::Endif) {
                    statements.push(self.recognize_statement());
                }

                self.match_token(Token::Endif);
                self.newline();

                Statement::If { comparisons, statements }
            },
            // "WHILE" comparisons nl "DO" nl {statement} nl "ENDWHILE" nl
            Some(Token::While) => {
                self.next_token();

                let start = self.counter;

                self.comparisons();
                self.newline();

                let comparisons = self.get_range_of_tokens(start, self.counter - 1);

                self.match_token(Token::Do);

                let mut statements: Vec<Statement> = Vec::new();
                self.match_token(Token::Newline);
                while self.current != Some(Token::Endwhile) {
                    statements.push(self.recognize_statement());
                }

                self.match_token(Token::Endwhile);
                self.newline();

                Statement::While { comparisons, statements }
            },
            // "INPUT" string identifier nl
            Some(Token::Input) => {
                self.next_token();

                self.match_token(Token::String(PLACEHOLDER, STRING_ID));
                let string = self.previous.clone().unwrap();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));
                let identifier = self.previous.clone().unwrap();

                self.newline();

                Statement::Input { string, identifier }
            },
            // "GOTO" identifier nl
            Some(Token::Goto) => {
                self.next_token();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));
                let identifier = self.previous.clone().unwrap();

                self.newline();

                Statement::Goto { identifier }
            },
            // nl ::= '\n'+
            Some(Token::Newline) => {
                self.next_token();

                // clear newlines till next different token
                if self.current == Some(Token::Newline) {
                    self.newline();
                }

                Statement::Empty
            },
            _ => {
                panic!("Syntax error: Token not recognized: {:#?}", self.current);
            }
        }
    }

    fn get_range_of_tokens(&mut self, start: usize, end: usize) -> Vec<Token> {
        const SUBTRACT_FROM_COUNTER: usize = 2;

        self.tokens.get((start - SUBTRACT_FROM_COUNTER)..(end - SUBTRACT_FROM_COUNTER)).unwrap().to_vec()
    }

    // comparisons ::= comparison {("AND" | "OR") comparison}
    fn comparisons(&mut self) {
        self.comparison();

        while self.current == Some(Token::And) || self.current == Some(Token::Or) {
            self.next_token();

            self.comparison();
        } 
    }

    // comparison ::= (expression equals expression) | ("true" | "false")
    fn comparison(&mut self) {
        if let Some(Token::Bool(_, BOOL_ID)) | Some(Token::Identifier(_, BOOL_ID)) = self.current {
            self.next_token();
        } else {
            self.expression();
        }
        
        self.equals();

        if let Some(Token::Bool(_, BOOL_ID)) | Some(Token::Identifier(_, BOOL_ID)) = self.current {
            self.next_token();
        } else {
            self.expression();
        }
    }

    // value ::= identifier | string | number | bool
    fn value(&mut self) {
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
        self.term();

         while self.current == Some(Token::Plus) || self.current == Some(Token::Minus) {
            self.next_token();

            self.term();
        }

    }

    // term ::= unary {("*" | "/" | "%") unary}
    fn term(&mut self) {
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
        if self.current == Some(Token::Plus) || self.current == Some(Token::Minus) {
            self.next_token();
        }

        self.primary();
    }

    // primary ::= identifier | number
    fn primary(&mut self) {
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
        self.previous.clone_from(&self.current);
        self.current.clone_from(&self.next);
        self.next = self.tokens.get(self.counter).cloned();
        self.counter += 1;
    }

    fn abort(message: String) {
        panic!("Syntax error: {}", message);
    }
}


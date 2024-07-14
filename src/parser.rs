use core::panic;

use crate::lexer::Token;
use crate::lexer::{
    // BOOL_ID, 
    STRING_ID, 
    NUMBER_ID, 
    IDENTIFIER_ID};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    // start: usize,
    counter: usize,
    // program: Vec<Token>,
    current: Option<Token>,
    next: Option<Token>,
}

const PLACEHOLDER: String = String::new();

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            // start: 0,
            counter: 0,
            // program: vec![],
            current: None,
            next: None,
        }
    }

    pub fn parse(&mut self) {
        // initialize
        self.next_token();
        self.next_token();

        while self.next.is_some() && self.counter <= self.tokens.len() {
            self.recognize_statement();
        }

        println!("Parsing done!");
    }

    // recognize grammar-tree statement
    fn recognize_statement(&mut self) {
        match self.current {
            // "PRINT" string nl
            Some(Token::Print) => {
                println!("Print-statement");
                self.next_token();

                self.match_token(Token::String(PLACEHOLDER, STRING_ID));
                // self.complex();

                self.newline();
            },
            // "LET" identifier "=" expression nl
            Some(Token::Let) => {
                println!("Let-statement");
                self.next_token();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));

                self.match_token(Token::Assign);

                self.expression();

                self.newline();
            },
            // "IF" comparison "THEN" nl {statement} nl "ENDIF" nl
            Some(Token::If) => {
                println!("If-statement");
                self.next_token();

                self.comparisons();
                self.match_token(Token::Then);

                self.match_token(Token::Newline);
                while self.current != Some(Token::Endif) {
                    self.recognize_statement();
                }

                self.match_token(Token::Endif);
                self.newline();
            },
            // "WHILE" comparison nl "DO" nl {statement} nl "ENDWHILE" nl
            Some(Token::While) => {
                println!("While-statement");
                self.next_token();

                self.comparisons();
                self.newline();
                self.match_token(Token::Do);

                self.match_token(Token::Newline);
                while self.current != Some(Token::Endwhile) {
                    self.recognize_statement();
                }

                self.match_token(Token::Endwhile);
                self.newline();
            },
            // "INPUT" string identifier nl
            Some(Token::Input) => {
                println!("Input-statement");
                self.next_token();

                self.match_token(Token::String(PLACEHOLDER, STRING_ID));
                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));
                self.newline();
            },
            // "GOTO" identifier nl
            Some(Token::Goto) => {
                println!("Goto-statement");
                self.next_token();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));
                self.newline();
            },
            // nl ::= '\n'+
            Some(Token::Newline) => {
                println!("Newline");
                self.next_token();

                // clear newlines till next different token
                if self.current == Some(Token::Newline) {
                    self.newline();
                }
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

        match self.current {
            Some(Token::Bool(_, _)) => {
                self.next_token();
                return;
            }
            _ => {}
        }

        self.expression();

        self.equals();

        self.expression();
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

    // complex ::= identifier | string
    fn complex(&mut self) {
        println!("Complex");

        match self.current {
            Some(Token::Identifier(_, id)) | Some(Token::String(_, id)) => {
                if id == IDENTIFIER_ID || id == STRING_ID { self.next_token(); return; }
            },
            _ => {}
        }

        Self::abort(
            format!(
                "expected {:#?} OR {:#?}, got {:#?}",
                Token::Identifier(PLACEHOLDER, IDENTIFIER_ID),
                Token::String(PLACEHOLDER, STRING_ID),
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
        self.current = self.next.clone();
        self.next = self.tokens.get(self.counter).cloned();
        self.counter += 1;
    }

    fn abort(message: String) {
        panic!("Syntax error: {}", message);
    }
}


use core::panic;
use std::collections::HashMap;
use crate::lexer::{Lexer, Token};
use crate::lexer::{
    BOOL_ID, 
    STRING_ID, 
    NUMBER_ID, 
    IDENTIFIER_ID};

#[derive(Debug)]
pub enum Statement {
    Print { value: Token },
    Let { identifier: Token, expression_tree: Expression },
    If { condition_tree: Condition, if_body: Vec<Statement> },
    While { condition_tree: Condition, while_body: Vec<Statement> },
    Input { message: Token, identifier: Token },
    Empty,
}


#[derive(PartialEq, Debug)]
pub enum Operand {
    Value { value: Token },
    Operation { operation: Box<Operation> },
}

#[derive(PartialEq, Debug)]
pub struct Operation {
    operand_left: Option<Operand>,
    operator: Option<Token>,
    operand_right: Option<Operand>,
}

#[derive(PartialEq, Debug)]
pub struct Condition {
    operation: Operation,
}

#[derive(PartialEq, Debug)]
pub enum Term {
    Value { sign: Option<Token>, value: Token},
    Operation { operation: Box<Expression> },
}

#[derive(PartialEq, Debug)]
pub struct Expression {
    left: Option<Term>,
    numeric_operator: Option<Token>,
    right: Option<Term>,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    counter: usize,
    current: Option<Token>,
    next: Option<Token>,
    previous: Option<Token>,
    used_identifiers: HashMap<String, Token>,
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
            used_identifiers: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        // initialize current, next, counter
        self.next_token();
        self.next_token();

        let mut statements: Vec<Statement> = vec![];

        while self.next.is_some() && self.counter <= self.tokens.len() {
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

                self.add_identifier(identifier.clone());

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
                let expression_tree = Self::expression_tree(&expression);

                self.newline();
                
                Statement::Let { identifier, expression_tree }
            },
            // "IF" comparisons "THEN" nl {statement} nl "ENDIF" nl
            Some(Token::If) => {
                self.next_token();

                let start = self.counter;

                self.comparisons();
                self.match_token(Token::Then);

                let condition_range = self.get_range_of_tokens(start, self.counter - 1);
                let condition_tree = Self::condition_tree(&condition_range);

                self.match_token(Token::Newline);

                let mut statements: Vec<Statement> = Vec::new();
                while self.current != Some(Token::Endif) {
                    statements.push(self.recognize_statement());
                }

                self.match_token(Token::Endif);
                self.newline();

                Statement::If { condition_tree, if_body: statements }
            },
            // "WHILE" comparisons nl "DO" nl {statement} nl "ENDWHILE" nl
            Some(Token::While) => {
                self.next_token();

                let start = self.counter;

                self.comparisons();
                self.newline();

                let condition_range = self.get_range_of_tokens(start, self.counter - 1);
                let condition_tree = Self::condition_tree(&condition_range);

                self.match_token(Token::Do);

                let mut statements: Vec<Statement> = Vec::new();
                self.match_token(Token::Newline);
                while self.current != Some(Token::Endwhile) {
                    statements.push(self.recognize_statement());
                }

                self.match_token(Token::Endwhile);
                self.newline();

                Statement::While { condition_tree, while_body: statements }
            },
            // "INPUT" string identifier nl
            Some(Token::Input) => {
                self.next_token();

                self.match_token(Token::String(PLACEHOLDER, STRING_ID));
                let message = self.previous.clone().unwrap();

                self.match_token(Token::Identifier(PLACEHOLDER, IDENTIFIER_ID));
                let identifier = self.previous.clone().unwrap();
                
                self.add_identifier(identifier.clone());

                self.newline();

                Statement::Input { message, identifier }
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

    fn condition_tree(condition: &[Token]) -> Condition {
        let operation = Self::operation_tree(condition);

        Condition { operation }
    }

    fn operation_tree(condition: &[Token]) -> Operation {
        let mut operation = Operation { 
            operand_left: None,
            operator: None,
            operand_right: None,
        };

        for (index, token) in condition.iter().enumerate() {
            let is_operand_and_left_is_empty = operation.operand_left.is_none() && Lexer::is_operand(token);
            let is_operator_and_middle_is_empty = operation.operator.is_none() && Lexer::is_operator(token);
            let is_operand_and_right_is_empty_and_is_last_token = operation.operand_right.is_none() && Lexer::is_operand(token) && index + 1 == condition.len();
            let is_operator_and_right_is_empty = operation.operand_right.is_none() && Lexer::is_operator(token);

            if is_operand_and_left_is_empty {
                operation.operand_left = Some(Operand::Value { value: token.clone() });
                continue;
            }
            if is_operator_and_middle_is_empty {
                operation.operator = Some(token.clone());
                continue;
            }
            if is_operand_and_right_is_empty_and_is_last_token {
                operation.operand_right = Some(Operand::Value { value: token.clone() });
                continue;
            }
            if is_operator_and_right_is_empty {
                // recursion here
                operation.operand_right = 
                    Some(Operand::Operation { operation:
                        Box::new(Self::operation_tree(
                            condition.get((index - 1)..condition.len())
                            .unwrap()
                        ))
                    });
            }
        }
        operation
    }

    fn expression_tree(expression: &[Token]) -> Expression {
        let mut expression_tree = Expression {
            left: None,
            numeric_operator: None,
            right: None,
        };

        let mut signed_token = None;

        for (index, token) in expression.iter().enumerate() {
            let is_sign_and_left_is_empty = expression_tree.left.is_none() && Lexer::is_sign(token);
            let is_operand_and_left_is_empty = expression_tree.left.is_none() && Lexer::is_operand(token);
            let is_numeric_and_numeric_field_is_empty = expression_tree.numeric_operator.is_none() && Lexer::is_numeric_operator(token);
            let is_sign_and_right_is_empty = expression_tree.right.is_none() && Lexer::is_sign(token);
            let is_operand_and_right_is_empty = expression_tree.right.is_none() && Lexer::is_operand(token);
            let is_last_token = index == (expression.len() - 1);

            // left
            if is_sign_and_left_is_empty {
                signed_token = Some(token);
                continue;
            }
            if is_operand_and_left_is_empty {
                expression_tree.left = Some(Term::Value { sign: signed_token.cloned(), value: token.clone() });
                signed_token = None;
                continue;
            }

            // middle
            if is_numeric_and_numeric_field_is_empty {
                expression_tree.numeric_operator = Some(token.clone());
                continue;
            }

            //right
            if is_sign_and_right_is_empty {
                signed_token = Some(token);
                continue;
            }
            if is_operand_and_right_is_empty && is_last_token {
                expression_tree.right = Some(Term::Value { sign: signed_token.cloned(), value: token.clone() });
                signed_token = None;
                continue;
            }
            // recursion here
            expression_tree.right =
                Some(Term::Operation { operation:
                    Box::new(Self::expression_tree(
                        expression.get((index - 1)..expression.len())
                        .unwrap()
                    ))
                });
        }

        expression_tree
    }

    fn check_identifier_from_string(&mut self, identifier: String) {
        if !self.used_identifiers.contains_key(&identifier) {
            panic!("Compile error: using uninitialized variable {}", identifier);
        }
    }

    fn add_identifier(&mut self, identifier: Token) {
        if let Token::Identifier(ref variable, _) = identifier {
            if !self.used_identifiers.contains_key(variable) {
                self.used_identifiers.insert(variable.to_string(), identifier);
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
        self.comparison_branch();
        
        self.equals();

        self.comparison_branch();
    }

    fn comparison_branch(&mut self) {
        match &self.current {
            Some(Token::Identifier(variable, IDENTIFIER_ID)) => {
                self.check_identifier_from_string(variable.to_string());
                self.expression();
            }
            Some(Token::Bool(_, BOOL_ID)) => self.next_token(),
            _ => self.next_token(),
        }
    }

    // value ::= identifier | string | number | bool
    fn value(&mut self) {
        match &self.current {
            Some(Token::Identifier(identifier, id)) if *id == IDENTIFIER_ID => {
                self.check_identifier_from_string(identifier.to_string());
                self.next_token(); return;
            },
            Some(Token::String(_, id))
            | Some(Token::Number(_, id))
            | Some(Token::Bool(_, id)) if 
                *id == IDENTIFIER_ID
                || *id == STRING_ID
                || *id == NUMBER_ID
                || *id == BOOL_ID => { 
                    self.next_token(); return;
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
        let mut is_matching = false;

        match &self.current {
            Some(Token::Identifier(identifier, id)) if *id == IDENTIFIER_ID => {
                self.check_identifier_from_string(identifier.to_string());
                self.next_token();
                is_matching = true;
            },
            Some(Token::Number(_, id)) if *id == NUMBER_ID => {
                self.next_token();
                is_matching = true;
            },
            _ => {}
        }

        if is_matching {
            return;
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

            _ if Some(&expected) != self.current.as_ref() => {
                    Self::abort(format!("expected {:#?}, got {:#?}", expected, self.current));
            }
            _ => {}
        }
        self.next_token();
    }

    // match special cases of tokens - Bool, String, Number, Identifier
    // do not check Identifier - already checked before every function use
    fn match_value_token(&mut self, expected: Token, id: u8) {
        match self.current {
            Some(Token::Bool(_, current_id)) | Some(Token::String(_, current_id)) |
            Some(Token::Number(_, current_id)) | Some(Token::Identifier(_, current_id))
            if id == current_id => {
                return;
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

impl Condition {
    pub fn get_tokens_from(&self) -> Vec<Token> {
        Condition::get_tokens_from_operation(&self.operation)
    }

    fn get_tokens_from_operation(o: &Operation) -> Vec<Token> {
        let mut tokens = vec![];
        
        // left
        if let Some(Operand::Value { value }) = &o.operand_left {
            tokens.push(value.clone());
        }
        if let Some(Operand::Operation { operation }) = &o.operand_left {
            tokens.append(&mut Condition::get_tokens_from_operation(operation));
        }

        // middle
        if let Some(operator) = &o.operator {
            tokens.push(operator.clone());
        }

        // right
        if let Some(Operand::Value { value }) = &o.operand_right {
            tokens.push(value.clone());
        }
        if let Some(Operand::Operation { operation }) = &o.operand_right{
            tokens.append(&mut Condition::get_tokens_from_operation(operation));
        }

        tokens
    }
}

impl Expression {
    pub fn get_tokens_from(&self) -> Vec<Token> {
        let mut tokens = vec![];
        
        // left
        if let Some(Term::Value { sign, value }) = &self.left {
            if sign.is_some() {
                tokens.push(sign.clone().unwrap());
            }
            tokens.push(value.clone());
        }
        if let Some(Term::Operation { operation }) = &self.left {
            tokens.append(&mut Expression::get_tokens_from(operation));
        }

        //middle
        if let Some(operator) = &self.numeric_operator {
            tokens.push(operator.clone());
        }

        // right
        if let Some(Term::Value { sign, value }) = &self.right {
            if sign.is_some() {
                tokens.push(sign.clone().unwrap());
            }
            tokens.push(value.clone());
        }
        if let Some(Term::Operation { operation }) = &self.right {
            tokens.append(&mut Expression::get_tokens_from(operation));
        }

        tokens
    }
}


use std::collections::HashMap;

use crate::lexer::{Lexer, Token};
use crate::parser::Statement;

#[derive(Debug)]
pub struct Emitter {
    statements: Vec<Statement>,
    identifiers: HashMap<String, u8>,
}

impl Emitter {
    pub fn new(statements: Vec<Statement>) -> Emitter {
        Emitter {
            statements,
            identifiers: HashMap::new(),
        }
    }

    pub fn emit(&mut self) -> Vec<String> {

        self.statements
            .iter()
            .map(|statement| self.evaluate(statement))
            .flatten()
            .collect()
    }

    fn evaluate(&self, statement: &Statement) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        match statement {
            Statement::Print { value } => {
                let variable_value = Self::unwrap_value_token(value.clone());

                output.push("println!(\"{}\", ".to_owned() + &variable_value + " );");
            }
            Statement::Let { identifier, expression } => {
                let variable = Self::unwrap_value_token(identifier.clone());
                let expr = Self::convert_expr_to_string(expression.clone());

                output.push("let mut ".to_owned() + &variable + " = " + &expr + ";");
            }
            Statement::If { comparisons, statements } => {
                let expr = Self::convert_expr_to_string(comparisons.clone());

                output.push("if ".to_string() + &expr + "{");

                let mut other_statements: Vec<String> = 
                statements
                    .iter()
                    .map(|s| { self.evaluate(s) })
                    .flatten()
                    .collect();

                output.append(&mut other_statements);

                output.push("}".to_string());
            }
            Statement::While { comparisons, statements } => {

            }
            Statement::Input { string, identifier } => {

            }
            Statement::Empty => {}
        }
        output
    }


    fn convert_expr_to_string(expression: Vec<Token>) -> String {
        let mut combined = String::new();                
        expression.iter().map(|token| { combined += &(Lexer::convert_token_to_string(token.clone()) + " ")});

        combined
    }

    fn unwrap_value_token(token: Token) -> String {
        if let Token::Bool(value, _) = token {
            return value.to_string();
        }

        if let Token::String(value, _) = token {
            return value;
        }

        if let Token::Number(value, _) = token {
            return value.to_string();
        }

        if let Token::Identifier(value, _) = token {
            return value.to_string();
        }
        
        panic!("Compile error: wrong value for token");
    }

    // FOR LATER
    //
    // analyze key points:
    // - follow identifiers (check types; if static, save as a number)
    // - if/while statements check for easier solutions, e.g. while x == true OR x > 1
    // - cache computed results
    // READ the wiki page about optimizing compiler
    // analysis
    pub fn analyze(&mut self) {}
    // optimization passes
    pub fn optimize(&mut self) {}
    // run file
    pub fn execute_code(&mut self) {}
}

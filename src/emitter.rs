use std::collections::HashMap;

use crate::lexer::{Lexer, Token};
use crate::parser::Statement;

#[derive(Debug)]
pub struct Emitter {
    statements: Vec<Statement>,
}

impl Emitter {
    pub fn new(statements: Vec<Statement>) -> Emitter {
        Emitter {
            statements,
        }
    }

    pub fn emit(&mut self) -> Vec<String> {
        let mut used_variables: HashMap<String, u8> = HashMap::new();

        self.statements
            .iter()
            .flat_map(|statement| Self::evaluate(statement, &mut used_variables))
            .collect()
    }

    fn evaluate(statement: &Statement, used_variables: &mut HashMap<String, u8>) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        match statement {
            Statement::Print { value } => {
                let variable_value = Self::unwrap_value_token(value.clone());

                output.push("println!(\"{}\", ".to_owned() + &variable_value + " );");
            }
            Statement::Let { identifier, expression } => {
                let variable = Self::unwrap_value_token(identifier.clone());
                let expr = Self::convert_expr_to_string(expression.clone());

                if used_variables.get(&variable).is_some() {
                    output.push(variable + " = " + &expr + ";");
                } else {
                    output.push("let mut ".to_owned() + &variable + " = " + &expr + ";");
                    used_variables.insert(variable, 0);
                }
            }
            Statement::If { comparisons, statements } => {
                let expr = Self::convert_expr_to_string(comparisons.clone());

                output.push("if ".to_owned() + &expr + " {");

                let mut other_statements: Vec<String> = 
                statements
                    .iter()
                    .flat_map(|s| { Self::evaluate(s, used_variables) })
                    .collect();

                output.append(&mut other_statements);

                output.push("}".to_string());
            }
            Statement::While { comparisons, statements } => {
                let expr = Self::convert_expr_to_string(comparisons.clone());

                output.push("while ".to_owned() + &expr + " {");

                let mut other_statements: Vec<String> = 
                statements
                    .iter()
                    .flat_map(|s| { Self::evaluate(s, used_variables) })
                    .collect();

                output.append(&mut other_statements);

                output.push("}".to_string());
            }
            Statement::Input { string, identifier } => {
                let text = Self::unwrap_value_token(string.clone());
                let variable = Self::unwrap_value_token(identifier.clone());

                if !used_variables.contains_key(&variable) {
                    output.push("let mut ".to_owned() + &variable + " = Default::default();");
                }
                output.push("println!(\"{}\", ".to_owned() + &text + ");");
                output.push("std::io::stdin().read_line(&mut ".to_owned() + &variable + ").expect(\"Failed to read user input\");");
            }
            Statement::Empty => {}
        }
        output
    }


    fn convert_expr_to_string(expression: Vec<Token>) -> String {
        let mut combined = String::new();                
        expression.iter().for_each(|token| { combined += &(Lexer::convert_token_to_string(token.clone()) + " ")});

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

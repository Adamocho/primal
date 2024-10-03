use std::collections::HashMap;
use crate::lexer::{Lexer, Token};
use crate::parser::{Condition, Expression, Statement};

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

        let mut lines = self.statements
            .iter()
            .flat_map(|statement| Self::evaluate(statement, &mut used_variables))
            .collect::<Vec<String>>();

        lines.insert(0, "#[allow(unused_mut)]".to_string());
        lines.insert(1, "fn main() {".to_string());
        
        lines.push("}".to_string());

        lines
    }

    fn evaluate(statement: &Statement, used_variables: &mut HashMap<String, u8>) -> Vec<String> {
        let mut output: Vec<String> = vec![];

        match statement {
            Statement::Print { value } => {
                let variable_value = Self::unwrap_value_token(value.clone());

                output.push("println!(\"{}\", ".to_owned() + &variable_value + " );");
            }
            Statement::Let { identifier, expression_tree } => {
                let variable = Self::unwrap_value_token(identifier.clone());
                let expr = Self::convert_expression_to_string(expression_tree);

                if used_variables.get(&variable).is_some() {
                    output.push(variable + " = " + &expr + ";");
                } else {
                    output.push("let mut ".to_owned() + &variable + " = " + &expr + ";");
                    used_variables.insert(variable, 0);
                }
            }
            Statement::If { condition_tree, if_body } => {
                let expr = Self::convert_condition_to_string(condition_tree);

                output.push("if ".to_owned() + &expr + " {");

                let mut other_statements: Vec<String> = 
                if_body
                    .iter()
                    .flat_map(|s| { Self::evaluate(s, used_variables) })
                    .collect();

                output.append(&mut other_statements);

                output.push("}".to_string());
            }
            Statement::While { condition_tree, while_body } => {
                let expr = Self::convert_condition_to_string(condition_tree);

                output.push("while ".to_owned() + &expr + " {");

                let mut other_statements: Vec<String> = 
                while_body
                    .iter()
                    .flat_map(|s| { Self::evaluate(s, used_variables) })
                    .collect();

                output.append(&mut other_statements);

                output.push("}".to_string());
            }
            Statement::Input { message, identifier } => {
                let text = Self::unwrap_value_token(message.clone());
                let variable = Self::unwrap_value_token(identifier.clone());

                if !used_variables.contains_key(&variable) {
                    output.push("let mut ".to_owned() + &variable + " = Default::default();");
                }
                output.push("println!(\"{}\", ".to_owned() + &text + ");");
                output.push("std::io::stdin().read_line(&mut ".to_owned() + &variable + ").expect(\"Failed to read user input\");");
                output.push("let mut ".to_owned() + &variable + " = user_input.trim().parse().unwrap();");
            }
            Statement::Empty => {}
        }
        output
    }
    
    fn convert_condition_to_string(condition: &Condition) -> String {
        let tokens = condition.get_tokens_from();

        Self::convert_expr_to_string(tokens)
    }

    fn convert_expression_to_string(expression: &Expression) -> String {
        let tokens = expression.get_tokens_from();

        Self::convert_expr_to_string(tokens)
    }

    fn convert_expr_to_string(expression: Vec<Token>) -> String {
        expression.iter().map(|token| Lexer::convert_token_to_string(token.clone())).collect::<Vec<String>>().join(" ")
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
}


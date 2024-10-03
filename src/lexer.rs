#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Print,
    Input,
    Let,
    If,
    Then,
    While,
    Do,
    End,
    Endif,
    Endwhile,

    Assign,

    Equals,
    NotEquals,
    MoreThanEquals,
    LessThanEquals,
    MoreThan,
    LessThan,
    And,
    Or,
    Not,

    Plus,
    Minus,
    Times,
    Divide,
    Modulo,

    // u8 is an ID for simpler matching
    Bool(bool, u8),
    String(String, u8),
    Number(i32, u8),
    Identifier(String, u8),

    Newline,
}

pub const BOOL_ID: u8 = 1;
pub const STRING_ID: u8 = 2;
pub const NUMBER_ID: u8 = 4;
pub const IDENTIFIER_ID: u8 = 8;


pub struct Lexer {}

impl Lexer {
    // main lexer function
    pub fn tokenize(contents: String) -> Vec<Token> {
        contents
            .lines()
            .map(Self::tokenize_line)
            .filter(|tokens| !tokens.is_empty()) // Care only about non-empty lines.
            .flatten() // possible thanks to Token::Newline
            .collect()
    }

    fn return_good_token(option_slice: Option<&str>) -> Vec<&str> {
        if let Some(unit) = option_slice { 
            return vec![unit];
        }
        vec![]
    }

    fn tokenize_line(line: &str) -> Vec<Token> {
        let mut lexemes: Vec<&str> = vec![];
        let mut lexeme_start = 0;
        let mut is_lexeme = false;
        let mut is_comment = false;
        let mut is_string = false;

        line.chars().enumerate().for_each(|(index, c)| {
            if is_comment { return; }

            match c {
                ' ' | '\n' if is_lexeme && !is_string => {
                    let mut token_vector = Self::return_good_token(line.get(lexeme_start..=index - 1));
                    lexemes.append(&mut token_vector);
                    
                    is_lexeme = false;
                }, 
                ' ' | '\n' => {}, // catch the rest of whitespace
                '#' => {
                    is_comment = true;
                },
                _ => {
                    if !is_lexeme {
                        is_lexeme = true;
                        lexeme_start = index;
                    }

                    if c == '"' {
                        is_string = !is_string;
                    }

                    if is_lexeme && index == line.len() - 1{
                        let mut token_vector = Self::return_good_token(line.get(lexeme_start..=index));
                        lexemes.append(&mut token_vector);

                        is_lexeme = false;
                    }
                }
            }
        });

        lexemes.push("\n"); // add NEWLINE at the end

        lexemes
            .iter()
            .map(|lexeme| {
                Self::identify_token(lexeme)
            })
            .collect()
    }

    fn is_valid_identifier(lexeme: &str) -> bool {
        for (index, c) in lexeme.chars().enumerate() {
            match c {
                // pretty much a hand-made regex for identifiers: [a-zA-Z_][a-zA-Z_0-9]*
                'a'..='z' | 'A'..='Z' | '_' => (),
                '0'..='9' if index != 0 => (),
                _ => { return false }
            }
        }
        true
    }

    fn is_valid_number(lexeme: &str) -> bool {
        lexeme.parse::<i32>().is_ok()
    }

    /// Rust automatically sanitises quotes -> \"text\"
    fn is_valid_string_literal(lexeme: &str) -> bool {
        if Some('"') == lexeme.chars().nth(0) && Some('"') == lexeme.chars().nth_back(0) {
            return true;
        }
        false
    }

    fn identify_token(lexeme: &str) -> Token {
        match lexeme {
            "\n" => Token::Newline,

            "PRINT" => Token::Print,
            "INPUT" => Token::Input,
            "LET" => Token::Let,
            "IF" => Token::If,
            "THEN" => Token::Then,
            "WHILE" => Token::While,
            "DO" => Token::Do,
            "END" => Token::End,
            "ENDIF" => Token::Endif,
            "ENDWHILE" => Token::Endwhile,
            "AND" => Token::And,
            "OR" => Token::Or,
            "NOT" => Token::Not,

            "=" => Token::Assign,

            "==" => Token::Equals,
            "!=" => Token::NotEquals,
            "=>" => Token::MoreThanEquals,
            ">" => Token::MoreThan,
            "<=" => Token::LessThanEquals,
            "<" => Token::LessThan,

            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Times,
            "/" => Token::Divide,
            "%" => Token::Modulo,

            "true" => Token::Bool(true, BOOL_ID),
            "false" => Token::Bool(false, BOOL_ID),

            x if Self::is_valid_string_literal(x)  => Token::String(x.to_string(), STRING_ID),
            x if Self::is_valid_number(x) => { Token::Number(x.parse().unwrap(), NUMBER_ID) },

            x if Self::is_valid_identifier(x) => Token::Identifier(x.to_string(), IDENTIFIER_ID),

            &_ => {
                panic!("Token not recognized: {:?}", lexeme);
            },
        }
    }

    pub fn convert_token_to_string(token: Token) -> String {
        match token {
            Token::Newline => "\n".to_string(),
            Token::Print => "PRINT".to_string(),
            Token::Input => "INPUT".to_string(),
            Token::Let => "LET".to_string(),
            Token::If => "IF".to_string(),
            Token::Then => "THEN".to_string(),
            Token::While => "WHILE".to_string(),
            Token::Do => "DO".to_string(),
            Token::End => "END".to_string(),
            Token::Endif => "ENDIF".to_string(),
            Token::Endwhile => "ENDWHILE".to_string(),
            Token::And => "&&".to_string(),
            Token::Or => "||".to_string(),
            Token::Not => "!".to_string(),
            Token::Assign => "=".to_string(),
            Token::Equals =>"==".to_string(),
            Token::NotEquals =>"!=".to_string(),
            Token::MoreThanEquals =>"=>".to_string(),
            Token::MoreThan => ">".to_string(),
            Token::LessThanEquals =>"<=".to_string(),
            Token::LessThan => "<".to_string(),
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Times => "*".to_string(),
            Token::Divide => "/".to_string(),
            Token::Modulo => "%".to_string(),
            Token::Bool(true, _) => "true".to_string(),
            Token::Bool(false, _) => "false".to_string(),
            Token::String(value, _) => value,
            Token::Number(value, _) => value.to_string(),
            Token::Identifier(value, _) => value,
        }
    }

    pub fn is_operator(token: &Token) -> bool {
        matches!(token, 
            Token::And |
            Token::Or |
            Token::Equals |
            Token::NotEquals |
            Token::MoreThanEquals |
            Token::MoreThan |
            Token::LessThanEquals |
            Token::LessThan |
            Token::Plus |
            Token::Minus |
            Token::Times |
            Token::Divide |
            Token::Modulo
        )
    }


    pub fn is_some_equality_operator(token: Option<&Token>) -> bool {
        if let Some(token) = token {
            return Self::is_equality_operator(token);
        }
        false
    }

    pub fn is_equality_operator(token: &Token) -> bool {
        matches!(token, 
            Token::Equals |
            Token::NotEquals |
            Token::MoreThanEquals |
            Token::MoreThan |
            Token::LessThanEquals |
            Token::LessThan
        )
    }

    pub fn is_some_logic_condition_operator(token: Option<&Token>) -> bool {
        if let Some(token) = token {
            return Self::is_logic_condition_operator(token);
        }
        false
    }

    pub fn is_logic_condition_operator(token: &Token) -> bool {
        matches!(token, 
            Token::And | Token::Or
        )
    }

    pub fn is_numeric_operator(token: &Token) -> bool {
        matches!(token,
            Token::Plus |
            Token::Minus |
            Token::Times |
            Token::Divide |
            Token::Modulo
        )
    }

    pub fn is_operand(token: &Token) -> bool {
        matches!(token,
            Token::Bool(_, _) |
            Token::String(_, _) |
            Token::Number(_, _) |
            Token::Identifier(_, _)
        )
    }

    pub fn is_sign(token: &Token) -> bool {
        matches!(token,
            Token::Plus |
            Token::Minus
        )
    }
}



#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn identify_string() {
        let x = "\"Hello\"with quotes\'in between\"";
        let result = self::Lexer::identify_token(x);

        assert_eq!(result, Token::String(x.to_string(), STRING_ID), "Couldn't identify a string");
    }

    #[test]
    #[should_panic]
    fn not_a_string() {
        let x = "\"I dont have an end quote..";

        // should panic here
        let _ = self::Lexer::identify_token(x);
    }

    #[test]
    fn identify_number() {
        let x: &str = "000345";

        let result = self::Lexer::identify_token(x);

        assert_eq!(result, Token::Number(x.parse().unwrap(), NUMBER_ID), "Couldn't identify a normal number");
    }

    #[test]
    #[should_panic]
    fn identify_a_float() {
        let x = "-10923,10293";
        let y = "999,420";

        let result1 = self::Lexer::identify_token(x);
        let result2 = self::Lexer::identify_token(y);

        assert_eq!(result1, Token::Number(x.parse().unwrap(), NUMBER_ID), "Couldn't identify a signed float");
        assert_eq!(result2, Token::Number(y.parse().unwrap(), NUMBER_ID), "Couldn't identify a float");
    }

    #[test]
    fn identify_identifier() {
        let x = "abcdefghijkmnlopqrstuvwxyzABCDEFGHIJKMNLOPQRSTUVWXYZ1234567890_";
        let result = self::Lexer::identify_token(x);

        assert_eq!(result, Token::Identifier(x.to_string(), IDENTIFIER_ID), "Identifier was not found");
    }

    #[test]
    #[should_panic]
    fn dont_identify_identifier() {
        // number at the start
        let x = "9_number";
        // wrong characters
        let y = "abc!this-is-bad+=!@#$%^&*";

        // should panic here
        let _ = self::Lexer::identify_token(x);
        let _ = self::Lexer::identify_token(y);
    }

    #[test]
    fn identify_a_line() {
        let line = "LET x = 15 \n";
        let result = self::Lexer::tokenize_line(line);

        assert_eq!(result, vec![Token::Let, Token::Identifier("x".to_string(), IDENTIFIER_ID), Token::Assign, Token::Number(15, NUMBER_ID), Token::Newline]);
    }

    #[test]
    fn identify_lines() {
        let lines = "LET x == IF \n WHILE PRINT = true \n";
        let result = self::Lexer::tokenize(lines.to_string());

        assert_eq!(result,
            vec![Token::Let,
                Token::Identifier("x".to_string(), IDENTIFIER_ID),
                Token::Equals,
                Token::If,
                Token::Newline,
                Token::While,
                Token::Print,
                Token::Assign,
                Token::Bool(true, BOOL_ID),
                Token::Newline
            ]);
    }
}


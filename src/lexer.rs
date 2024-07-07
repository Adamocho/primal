use core::panic;


// 2 phases
// DONE // scanner -> [(identifier, x), (literal, "hello")]
// evaluator -> IDENTIFIER x    LITERAL_STRING "hello"

#[derive(Debug)]
pub enum Token {
    Print,
    Let,
    If,
    Then,
    While,
    Do,
    End,
    Endif,
    Endwhile,

    Equals,
    Assign,
    MoreThanEquals,
    LessThanEquals,
    MoreThan,
    LessThan,
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,

    Bool(bool),
    String(String),
    Number(i32),

    Identifier(String),

    // Is this necessary?
    Unknown,
}

pub struct Lexer {}

impl Lexer {
    // main lexer function
    pub fn tokenize(contents: String) -> Vec<Vec<Token>> {
        contents
            .lines()
            .map(|line| Self::tokenize_line(line))
            .filter(|tokens| !tokens.is_empty()) // Care only about non-empty lines.
            .collect()
    }

    fn tokenize_line(line: &str) -> Vec<Token> {

        let mut lexemes: Vec<&str> = vec![];
        let mut lexeme_start = 0;
        let mut is_lexeme = false;
        let mut is_comment = false;
        let mut is_string = false;

        line.chars().enumerate().for_each(|(index, c)| {

            // ignore the rest of the line
            if is_comment { return; }

            match c {
                ' ' | '\n' => {
                    if is_lexeme && !is_string {
                        if let Some(unit) = line.get(lexeme_start..=index - 1) { 
                            lexemes.push(unit);
                        }
                        is_lexeme = false;
                    }
                },
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
                        if let Some(unit) = line.get(lexeme_start..=index) { 
                            lexemes.push(unit);
                        }
                        is_lexeme = false;
                    }
                }
            }
        });

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
        match lexeme.parse::<i32>() {
            Ok(_) => true,
            _ => false
        }
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
                // just to be sure it doesn't panic for now
            // "" => Token::Unknown,
            // "\n" => Token::Unknown,
            // " " => Token::Unknown,

            "PRINT" => Token::Print,
            "LET" => Token::Let,
            "IF" => Token::If,
            "THEN" => Token::Then,
            "WHILE" => Token::While,
            "DO" => Token::Do,
            "END" => Token::End,
            "ENDIF" => Token::Endif,
            "ENDWHILE" => Token::Endwhile,

            "==" => Token::Equals,
            "=" => Token::Assign,
            "=>" => Token::MoreThanEquals,
            ">" => Token::MoreThan,
            "<=" => Token::LessThanEquals,
            "<" => Token::LessThan,
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Times,
            "/" => Token::Divide,
            "%" => Token::Modulo,

            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            x if Self::is_valid_string_literal(x)  => Token::String(x.to_string()),
            x if Self::is_valid_number(x) => { Token::Number(x.parse().unwrap()) },

            x if Self::is_valid_identifier(x) => Token::Identifier(x.to_string()),

            &_ => {
                panic!("Token not recognized: {:?} - {:?}", lexeme, Token::Unknown);
            },
        }
    }
}


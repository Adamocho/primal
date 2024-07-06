use core::panic;

#[derive(Debug)]
enum Lexeme {
    Identifier,
    Operator,
    Keyword,
    NumberLiteral,
    StringLiteral,
    BoolLiteral,
    // Comment,
    Whitespace,
}

// generic type needed
enum Token {
}

// 2 phases
// scanner -> [(identifier, x), (literal, "hello")]
// evaluator -> IDENTIFIER x    LITERAL_STRING "hello"

pub struct Lexer {
    output: String,
    line_counter: u32,
    character_counter: u32,
    scann: Vec<(Lexeme, String)>,
    evaluation: Vec<Token>
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            output: String::new(),
            character_counter: 0,
            line_counter: 0,
            evaluation: Vec::new(),
            scann: Vec::new(),
        }
    }

    // main lexer function
    pub fn tokenize(&mut self, contents: String) {
        let all_tokens: Vec<Vec<Lexeme>> = contents.lines().map(|line| self.tokenize_line(line)).collect();
    }

    fn tokenize_line(&mut self, line: &str) -> Vec<Lexeme> {

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
                self.identify_token(lexeme)
            }).collect()
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

    fn identify_token(&mut self, lexeme: &str) -> Lexeme {
        match lexeme {
            // just to be sure it doesn't panic
            "" => Lexeme::Whitespace,
            "\n" => Lexeme::Whitespace,
            " " => Lexeme::Whitespace,

            "PRINT" => Lexeme::Keyword,
            "LET" => Lexeme::Keyword,
            "IF" => Lexeme::Keyword,
            "THEN" => Lexeme::Keyword,
            "WHILE" => Lexeme::Keyword,
            "DO" => Lexeme::Keyword,
            "END" => Lexeme::Keyword,
            "ENDIF" => Lexeme::Keyword,
            "ENDWHILE" => Lexeme::Keyword,

            "=" => Lexeme::Operator,
            ">" => Lexeme::Operator,
            "<" => Lexeme::Operator,
            "+" => Lexeme::Operator,
            "-" => Lexeme::Operator,
            "*" => Lexeme::Operator,
            "/" => Lexeme::Operator,
            "%" => Lexeme::Operator,

            "true" => Lexeme::BoolLiteral,
            "false" => Lexeme::BoolLiteral,
            x if Self::is_valid_string_literal(x)  => Lexeme::StringLiteral,
            x if Self::is_valid_number(x) => Lexeme::NumberLiteral,

            x if Self::is_valid_identifier(x) => Lexeme::Identifier,
            x if Self::is_valid_identifier(x) => Lexeme::Identifier,

            &_ => {
                panic!("Token not recognized: {lexeme}")
            }
        }
    }
}


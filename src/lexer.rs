use core::panic;


/// Flages used:
/// 0 0 0 0 _ 0 0 0 0
///                 ^
/// Does it a comment? (Discard the rest of the line) 
type Flags = u8; // 0000_0000 - eight flags to choose from


#[derive(Debug)]
enum Lexeme {
    Identifier,
    Operator,
    Keyword,
    Literal,
    Comment,
    Whitespace,
}

// generic type needed
enum Token {
}

// 2 phases
// scanner -> [(identifier, x), (literal, "hello")]
// evaluator -> IDENTIFIER x    LITERAL_STRING "hello"

pub struct Lexer {
    flags: Flags,
    output: String,
    line_counter: u32,
    character_counter: u32,
    scann: Vec<(Lexeme, String)>,
    evaluation: Vec<Token>
}

impl Lexer {
    pub fn new(flags: Flags) -> Self {
        Lexer {
            flags,
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
        dbg!(all_tokens);
    }
    
    fn tokenize_line(&mut self, line: &str) -> Vec<Lexeme> {
        let lexemes: Vec<Lexeme> = line.split(' ')
            // .filter(|_| self.flags & 0 == 0)
            .map(|lexeme| {
                self.identify_lexeme(lexeme)
            }).collect();

        // reset IS_COMMENT flag
        self.flags &= 1;

        vec![]
    }

    fn process_character(&mut self, c: char) -> Lexeme {
        panic!("Character not recognized: {c}")
    }

    // look at the next character
    fn lookup_ahead(&mut self) {
    }

    // pretty much a hand-made regex for identifiers: [a-zA-Z_][a-zA-Z_0-9]*
    fn is_valid_identifier(lexeme: &str) -> bool {
        ill do this tomorrow
    }


    fn identify_lexeme(&mut self, lexeme: &str) -> Lexeme {%
        match lexeme {
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

            ">" => Lexeme::Operator,
            "<" => Lexeme::Operator,
            "+" => Lexeme::Operator,
            "-" => Lexeme::Operator,
            "*" => Lexeme::Operator,
            "/" => Lexeme::Operator,
            "%" => Lexeme::Operator,
            "#" => {
                self.flags |= 1;
                Lexeme::Comment
            },

            "true" => Lexeme::Literal,
            "false" => Lexeme::Literal,
            x if { 
                let mut xs = x.chars();
                // check first and last for string
                xs.next() == Some('"') && xs.next_back() == Some('"') 
            }  => Lexeme::Literal,
            x if {
                match x.parse::<i32>() {
                    Ok(_) => true,
                    _ => false
                }
            } => Lexeme::Literal,
            x if Self::is_valid_identifier(x) => Lexeme::Literal,

            &_ => {
                panic!("Pattern not recognized: {lexeme}")
            }
        }
    }
}



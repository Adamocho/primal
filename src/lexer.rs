use core::panic;


/// Flags:
/// 0b_0 0 0 0 0 0 0 0
/// id 0 1 2 3 4 5 6 7
///    ^ ^ ^ ^
///    | | | IS_COMMENT (Discard the rest of the line) 
///    | | IS_PART_STRING (continue till the other double quote '"' is found)
///    | IS_ESCAPED (quote)
///    IS_LEXEME


type Flag = u8; // 0000_0000 - eight flags to choose from

enum Flags {
    IsLexeme = 0,
    IsEscaped = 1,
    IsPartString = 2,
    IsComment = 3,
}




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
    flags: Flag,
    output: String,
    line_counter: u32,
    character_counter: u32,
    scann: Vec<(Lexeme, String)>,
    evaluation: Vec<Token>
}

impl Lexer {
    pub fn new(flags: Flag) -> Self {
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
    }

    fn tokenize_line(&mut self, line: &str) -> Vec<Lexeme> {

        let mut lexemes: Vec<&str> = vec![];
        let mut lexeme_start = 0;
        let mut counter = 0; 

        line.chars().enumerate().for_each(|(index, c)| {

            dbg!(self.flags, index, c);

            // ignore the rest of the line
            if self.get_flag(Flags::IsComment as u8) { return; }

            match c {
                ' ' | '\n' => {
                    if self.get_flag(Flags::IsComment as u8) {
                        counter += 1;

                        if let Some(unit) = line.get(lexeme_start..=index - 1) { 
                            lexemes.push(unit);
                        }
                        self.flip_flag(Flags::IsLexeme as u8);
                    }
                },
                '#' => {
                    self.flip_flag(Flags::IsComment as u8);
                    return;
                },
                _ => {
                    if !self.get_flag(Flags::IsLexeme as u8) {
                        self.flip_flag(Flags::IsLexeme as u8);
                        lexeme_start = index;
                    }
                }
            }
        });

        // reset used flags
        self.flags &= 0b0000_1111;

        dbg!(counter);
        dbg!(&lexemes);

        panic!("Panic");

        vec![]

        // lexemes
        //     .iter()
        //     .map(|lexeme| {
        //         self.identify_lexeme(lexeme)
        //     }).collect()
    }

    fn get_flag(&self, flag_number: u8) -> bool {
        ((self.flags >> (7 - flag_number)) & 0b1) == 1
    }

    fn flip_flag(&mut self, flag_number: u8) {
        self.flags ^= 1 << (7 - flag_number);
    }

    fn set_flag(&mut self, flag_number: u8, value: bool) {
        let flag = self.get_flag(flag_number);

        if flag == value { return; }
        self.flip_flag(flag_number);
    }

    fn process_character(&mut self, c: char) -> Lexeme {
        panic!("Character not recognized: {c}")
    }

    // look at the next character
    fn lookup_ahead(&mut self) {
    }

    fn is_valid_identifier(lexeme: &str) -> bool {
        for (index, c) in lexeme.chars().enumerate() {
            match c {
                // pretty much a hand-made regex for identifiers: [a-zA-Z_][a-zA-Z_0-9]*
                'a'..='z' | 'A'..='Z' | '_' => (),
                '0'..='9' if index != 0 => (),
                _ => {dbg!(index, c); return false;}
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

    fn is_string_literal(lexeme: &str) -> bool {
        return true;
        // todo!("Not yet implemented");
    }

    fn quote_found(&mut self) {
        // flip the second bit
        self.flags ^= 10;
    }

    fn identify_lexeme(&mut self, lexeme: &str) -> Lexeme {
        match lexeme {
            "\"" => {
                self.quote_found();
                Lexeme::Literal
            },

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
            "#" => {
                self.flags |= 1;
                Lexeme::Comment
            },

            "true" => Lexeme::Literal,
            "false" => Lexeme::Literal,
            x if Self::is_string_literal(x)  => Lexeme::Literal,
            x if Self::is_valid_number(x) => Lexeme::Literal,
            x if Self::is_valid_identifier(x) => Lexeme::Literal,

            &_ => {
                panic!("Pattern not recognized: {lexeme}")
            }
        }
    }
}



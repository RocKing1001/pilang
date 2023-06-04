#[derive(Debug, PartialEq)]
pub enum Tokens {
    ILLEGAL,
    EOF,
    INDENDATION,

    // Identifiers + literals
    IDENT(String),
    INT(i32),

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION, // fn
    LET,      // let
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.move_forward();

        return lexer;
    }

    pub fn move_forward(&mut self) {
        self.ch = match self.input.chars().nth(self.read_position) {
            Some(ch) => ch,
            None => '\0',
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Tokens {
        let mut dont_move = false;
        let token = match self.ch {
            '=' => Tokens::ASSIGN,
            ';' => Tokens::SEMICOLON,
            '(' => Tokens::LPAREN,
            ')' => Tokens::RPAREN,
            ',' => Tokens::COMMA,
            '+' => Tokens::PLUS,
            '{' => Tokens::LBRACE,
            '}' => Tokens::RBRACE,
            '\0' => Tokens::EOF,
            _ => {
                if self.ch.is_whitespace() {
                    Tokens::INDENDATION
                } else if let Some(int) = self.read_int() {
                    dont_move = true;
                    Tokens::INT(int)
                } else if self.is_letter() {
                    dont_move = true;
                    let identifier = self.read_identifier();
                    match identifier.as_str() {
                        "fn" => Tokens::FUNCTION,
                        "let" => Tokens::LET,
                        _ => Tokens::IDENT(identifier),
                    }
                } else {
                    Tokens::ILLEGAL
                }
            }
        };
        if !dont_move {
            self.move_forward();
        }
        return token;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        while self.is_letter() {
            self.move_forward();
        }

        return self.input[position..self.position].to_string();
    }

    fn read_int(&mut self) -> Option<i32> {
        let position = self.position;

        while self.ch.is_digit(10) {
            self.move_forward();
        }

        return self.input[position..self.position].parse::<i32>().ok();
    }

    fn is_letter(&self) -> bool {
        return self.ch >= 'a' && self.ch <= 'z'
            || self.ch >= 'A' && self.ch <= 'Z'
            || self.ch == '_';
    }
}

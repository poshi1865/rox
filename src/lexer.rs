use core::fmt;
use std::char;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, Read};

#[allow(dead_code)]
enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    // Invalid
    Invalid
}

fn report_error(line: usize, message: String) {
    eprintln!("Error occured on line {}, {}", line, message);
    std::process::exit(65);
}

#[derive(Debug)]
enum RoxError {
    InvalidToken,
    IOError(io::Error),
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize
}

struct Lexer {
    chars: Vec<char>,
    pointer: usize,
    current_line: usize,
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let next_token: Token;
        match self.chars.get(self.pointer) {
            Some(c) => {
                // Single character tokens
                let lexeme = c.to_string();
                let line = self.current_line;
                match c {
                    '(' => next_token = Token {token_type: TokenType::LeftParen, lexeme: lexeme, line: line},
                    ')' => next_token = Token {token_type: TokenType::RightParen, lexeme: lexeme, line: line},
                    '{' => next_token = Token {token_type: TokenType::LeftBrace, lexeme: lexeme, line: line},
                    '}' => next_token = Token {token_type: TokenType::RightBrace, lexeme: lexeme, line: line},
                    ',' => next_token = Token {token_type: TokenType::Comma, lexeme: lexeme, line: line},
                    '.' => next_token = Token {token_type: TokenType::Dot, lexeme: lexeme, line: line},
                    '-' => next_token = Token {token_type: TokenType::Minus, lexeme: lexeme, line: line},
                    '+' => next_token = Token {token_type: TokenType::Plus, lexeme: lexeme, line: line},
                    ';' => next_token = Token {token_type: TokenType::Semicolon, lexeme: lexeme, line: line},
                    '*' => next_token = Token {token_type: TokenType::Star, lexeme: lexeme, line: line},
                    '\n' => {
                        next_token = Token {token_type: TokenType::Invalid, lexeme: lexeme, line: line};
                        self.current_line += 1;
                    },
                    _ => next_token = Token {token_type: TokenType::Invalid, lexeme: lexeme, line: line},
                }
            },
            None => {
                return None
            }
        }
        self.pointer += 1;
        return Some(next_token);
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>)-> Result<(), std::fmt::Error> {
        println!("Token: {}, Line: {}", self.lexeme, self.line);
        Ok(())
    }
}

impl Lexer {
    fn new(source_file_path: String) -> Result<Lexer, io::Error> {
        let mut file = File::open(source_file_path)?;
        let mut source_buffer: String = String::new();
        file.read_to_string(&mut source_buffer)?;

        let chars: Vec<char> = source_buffer.chars().collect();

        Ok(Lexer {
            chars: chars,
            pointer: 0,
            current_line: 0,
        })
    }
}

fn tokenize(source_file_path: String) -> Result<Vec<Token>, RoxError> {
    let mut lexer = Lexer::new(source_file_path).map_err(RoxError::IOError)?;
    let mut tokens: Vec<Token> = Vec::new();

    for token in lexer.into_iter() {
        tokens.push(token);
    }
    return Ok(tokens);
}

pub fn print_tokens(source_file_path: String) {
    let tokens = tokenize(source_file_path).unwrap();
    for token in tokens {
        println!("{}", token);
    }
}

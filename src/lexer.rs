use std::char;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, Read};

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Keyword,

    // Error.
    Error
}

fn report_error(line: usize, message: String) {
    eprintln!("Error occured on line {}, {}", line, message);
    std::process::exit(65);
}

#[derive(Debug)]
pub enum RoxError {
    InvalidToken,
    IOError(io::Error),
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    line: usize,
}

pub struct Lexer {
    chars: Vec<char>,
    pointer: usize,
    current_line: usize,
    keyword_list: Vec<String>,
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let mut next_token: Option<Token> = Option::None;

        // A loop here makes it so that I don't have to return the next_token in every single
        // arm. I can 'continue' in certain arms like whitespaces.
        loop {
            match self.chars.get(self.pointer) {
                Some(c) => {
                    // Single character tokens
                    let lexeme = c.to_string();
                    let line = self.current_line;
                    match c {
                        '(' => {
                            next_token = Some(Token {
                                token_type: TokenType::LeftParen,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        ')' => {
                            next_token = Some(Token {
                                token_type: TokenType::RightParen,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        '{' => {
                            next_token = Some(Token {
                                token_type: TokenType::LeftBrace,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        '}' => {
                            next_token = Some(Token {
                                token_type: TokenType::RightBrace,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        ',' => {
                            next_token = Some(Token {
                                token_type: TokenType::Comma,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        '.' => {
                            next_token = Some(Token {
                                token_type: TokenType::Dot,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        '-' => {
                            next_token = Some(Token {
                                token_type: TokenType::Minus,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        '+' => {
                            next_token = Some(Token {
                                token_type: TokenType::Plus,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        ';' => {
                            next_token = Some(Token {
                                token_type: TokenType::Semicolon,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        '*' => {
                            next_token = Some(Token {
                                token_type: TokenType::Star,
                                lexeme: lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        '=' => {
                            if self.peek_one_char() == '=' {
                                next_token = Some(Token {
                                    token_type: TokenType::EqualEqual,
                                    lexeme: "==".to_string(),
                                    line: line,
                                });
                                self.pointer += 2;
                            } else {
                                next_token = Some(Token {
                                    token_type: TokenType::Equal,
                                    lexeme: "=".to_string(),
                                    line: line,
                                });
                                self.pointer += 1;
                            }
                        }
                        '<' => {
                            if self.peek_one_char() == '=' {
                                next_token = Some(Token {
                                    token_type: TokenType::LessEqual,
                                    lexeme: "<=".to_string(),
                                    line: line,
                                });
                                self.pointer += 2;
                            } else {
                                next_token = Some(Token {
                                    token_type: TokenType::Less,
                                    lexeme: "<".to_string(),
                                    line: line,
                                });
                                self.pointer += 1;
                            }
                        }
                        '>' => {
                            if self.peek_one_char() == '=' {
                                next_token = Some(Token {
                                    token_type: TokenType::GreaterEqual,
                                    lexeme: ">=".to_string(),
                                    line: line,
                                });
                                self.pointer += 2;
                            } else {
                                next_token = Some(Token {
                                    token_type: TokenType::Greater,
                                    lexeme: ">".to_string(),
                                    line: line,
                                });
                                self.pointer += 1;
                            }
                        }
                        '!' => {
                            if self.peek_one_char() == '=' {
                                next_token = Some(Token {
                                    token_type: TokenType::BangEqual,
                                    lexeme: "!=".to_string(),
                                    line: line,
                                });
                                self.pointer += 2;
                            } else {
                                next_token = Some(Token {
                                    token_type: TokenType::Bang,
                                    lexeme: lexeme,
                                    line: line,
                                });
                                self.pointer += 1;
                            }
                        }
                        '/' => {
                            if self.peek_one_char() == '/' {
                                while let Some(current_char) = self.chars.get(self.pointer) {
                                    if current_char == &'\n' {
                                        break;
                                    }
                                    self.pointer += 1;
                                }
                            }
                            else {
                                next_token = Some(Token {
                                    token_type: TokenType::Slash,
                                    lexeme: lexeme,
                                    line: line,
                                });
                                self.pointer += 1;
                            }
                        }
                        '"' => {
                            self.pointer += 1;
                            let mut string_lexeme = String::from("\"");
                            let mut current_char: Option<&char> = self.chars.get(self.pointer);

                            loop {
                                match current_char {
                                    Some(c) => {
                                        string_lexeme.push(c.clone());
                                        if c == &'"' {
                                            break;
                                        }
                                        self.pointer += 1;
                                        current_char = self.chars.get(self.pointer);
                                    }
                                    None => return None,
                                }
                            }
                            next_token = Some(Token {
                                token_type: TokenType::String,
                                lexeme: string_lexeme,
                                line: line,
                            });
                            self.pointer += 1;
                        }
                        ' ' | '\t' | '\n' => {
                            while let Some(c) = self.chars.get(self.pointer) {
                                if c == &'\n' {
                                    self.current_line += 1;
                                }
                                else if c != &' ' && c != &'\t' {
                                    break;
                                }
                                self.pointer += 1;
                            }
                            continue;
                        }
                        other => {
                            // At this point, its either an identifier, a digit or a keyword
                            let mut current_char: Option<&char> = Some(other);
                            let mut word = String::new();
                            loop {
                                match current_char {
                                    Some(c) => {
                                        if c == &' ' || c == &'\n' || c == &'\t' {
                                            break;
                                        }
                                        self.pointer += 1;
                                        word.push(c.clone());
                                        current_char = self.chars.get(self.pointer);
                                    }
                                    None => return None,
                                }
                            }

                            if word.parse::<f64>().is_ok() {
                                next_token = Some(Token {
                                    token_type: TokenType::Number,
                                    lexeme: word,
                                    line: line,
                                });
                            } else if self.keyword_list.contains(&word) {
                                next_token = Some(Token {
                                    token_type: TokenType::Keyword,
                                    lexeme: word,
                                    line: line,
                                });
                            } else {
                                next_token = Some(Token {
                                    token_type: TokenType::Identifier,
                                    lexeme: word,
                                    line: line,
                                });
                            }
                        }
                    }
                }
                None => return None,
            }
            return next_token;
        } //loop end
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let ttype: &str;
        match self.token_type {
            // Single-character tokens.
            TokenType::LeftParen => ttype = "LeftParen",
            TokenType::RightParen => ttype = "RightParen",
            TokenType::LeftBrace => ttype = "LeftBrace",
            TokenType::RightBrace => ttype = "RightBrace",
            TokenType::Comma => ttype = "Comma",
            TokenType::Dot => ttype = "Dot",
            TokenType::Minus => ttype = "Minus",
            TokenType::Plus => ttype = "Plus",
            TokenType::Semicolon => ttype = "Semicolon",
            TokenType::Slash => ttype = "Slash",
            TokenType::Star => ttype = "Star",

            TokenType::Bang => ttype = "Bang",
            TokenType::BangEqual => ttype = "BangEqual",
            TokenType::Equal => ttype = "Equal",
            TokenType::EqualEqual => ttype = "EqualEqual",
            TokenType::Greater => ttype = "Greater",
            TokenType::GreaterEqual => ttype = "GreaterEqual",
            TokenType::Less => ttype = "Less",
            TokenType::LessEqual => ttype = "LessEqual",

            TokenType::Identifier => ttype = "Identifier",
            TokenType::String => ttype = "String",
            TokenType::Number => ttype = "Number",

            // Keywords.
            TokenType::And => ttype = "And",
            TokenType::Class => ttype = "Class",
            TokenType::Else => ttype = "Else",
            TokenType::False => ttype = "False",
            TokenType::Fun => ttype = "Fun",
            TokenType::For => ttype = "For",
            TokenType::If => ttype = "If",
            TokenType::Nil => ttype = "Nil",
            TokenType::Or => ttype = "Or",
            TokenType::Print => ttype = "Print",
            TokenType::Return => ttype = "Return",
            TokenType::Super => ttype = "Super",
            TokenType::This => ttype = "This",
            TokenType::True => ttype = "True",
            TokenType::Var => ttype = "Var",
            TokenType::While => ttype = "While",
            TokenType::Keyword => ttype = "Keyword",

            // Error.
            TokenType::Error => ttype = "Error",
        }
        println!("Type: {} | Line: {} | Lexeme: {}", ttype, self.line, self.lexeme);
        Ok(())
    }
}

impl Lexer {
    pub fn new(source_file_path: String) -> Result<Lexer, io::Error> {
        let mut file = File::open(source_file_path)?;
        let mut source_buffer: String = String::new();
        file.read_to_string(&mut source_buffer)?;

        let chars: Vec<char> = source_buffer.chars().collect();

        Ok(Lexer {
            chars: chars,
            pointer: 0,
            current_line: 0,
            keyword_list: vec![
                "and".to_string(),
                "class".to_string(),
                "else".to_string(),
                "false".to_string(),
                "for".to_string(),
                "fun".to_string(),
                "if".to_string(),
                "nil".to_string(),
                "or".to_string(),
                "return".to_string(),
                "super".to_string(),
                "this".to_string(),
                "true".to_string(),
                "var".to_string(),
                "while".to_string(),
                "print".to_string(),
            ],
        })
    }

    // This is actually small enough to not be a function anymore
    fn peek_one_char(&self) -> char {
        // Peeks the next char.
        // TODO: self.chars[p] can go out of bounds and panic. Handle cleanly.
        return self.chars[self.pointer + 1];
    }
}

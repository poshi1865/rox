use std::iter::Peekable;

use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;

enum Expr {
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>,
    },
    Unary {
        op: TokenType,
        right: Box<Expr>,
    },
    Literal {
        value: String,
    },
    Grouping {
        expr: Box<Expr>
    }
}

struct Parser<'a> {
    lexer: Peekable<&'a mut Lexer>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        // let first_token = lexer.next().expect("Lexer does not contain any tokens!");
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn expr(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let expr: Expr = self.comparison()?;

        let peeked_token: Option<&Token> = self.lexer.peek();
        match peeked_token {
            Some(t) => {
                if t.token_type == TokenType::BangEqual || t.token_type == TokenType::EqualEqual {
                    let expr_right = self.comparison()?;
                    return Ok(Expr::Binary {
                        left: Box::new(expr),
                        op: self.lexer.next().unwrap().token_type,
                        right: Box::new(expr_right),
                    });
                } else {
                    return Ok(expr);
                }
            }
            None => {
                todo!()
            }
        }
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let expr: Expr = self.term()?;
        let peeked_token = self.lexer.peek();
        match peeked_token {
            Some(t) => {
                if t.token_type == TokenType::Greater
                    || t.token_type == TokenType::GreaterEqual
                    || t.token_type == TokenType::Less
                    || t.token_type == TokenType::LessEqual
                {
                    let expr_right = self.term()?;
                    return Ok(Expr::Binary {
                        left: Box::new(expr),
                        op: self.lexer.next().unwrap().token_type,
                        right: Box::new(expr_right),
                    });
                } else {
                    return Ok(expr);
                }
            }
            None => {
                todo!()
            }
        }
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let expr: Expr = self.factor()?;
        let peeked_token = self.lexer.peek();
        match peeked_token {
            Some(t) => {
                if t.token_type == TokenType::Minus || t.token_type == TokenType::Plus {
                    let expr_right = self.factor()?;
                    return Ok(Expr::Binary {
                        left: Box::new(expr),
                        op: self.lexer.next().unwrap().token_type,
                        right: Box::new(expr_right),
                    });
                } else {
                    return Ok(expr);
                }
            }
            None => {
                todo!()
            }
        }
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let expr: Expr = self.unary()?;
        let peeked_token = self.lexer.peek();
        match peeked_token {
            Some(t) => {
                if t.token_type == TokenType::Slash || t.token_type == TokenType::Star {
                    let expr_right = self.unary()?;
                    return Ok(Expr::Binary {
                        left: Box::new(expr),
                        op: self.lexer.next().unwrap().token_type,
                        right: Box::new(expr_right),
                    });
                } else {
                    return Ok(expr);
                }
            }
            None => {
                todo!()
            }
        }
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        let current_token = self.lexer.next();
        match current_token {
            Some(t) => {
                if t.token_type == TokenType::Bang || t.token_type == TokenType::Minus {
                    let expr_right = self.unary()?;
                    return Ok(Expr::Unary {
                        op: t.token_type,
                        right: Box::new(expr_right),
                    });
                } else {
                    return self.primary();
                }
            }
            None => {
                todo!()
            }
        }
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        let current_token = self.lexer.next();
        match current_token {
            Some(t) => {
                if t.token_type == TokenType::LeftParen {
                    let expr = self.expr()?;
                    // Consume the closing paranthesis
                    self.lexer.next();
                    return Ok(Expr::Grouping { expr: Box::new(expr) });
                }
                else {
                    return Ok(Expr::Literal { value: t.lexeme });
                }
            },
            None => {todo!()}
        }
    }
}




















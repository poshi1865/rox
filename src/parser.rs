use std::iter::Peekable;

use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::lexer::TokenType;

enum Expr {
    Binary {
        left: Box<Expr>,
        op: TokenType,
        right: Box<Expr>
    },
}

struct Parser<'a> {
    lexer: Peekable<&'a mut Lexer>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        // let first_token = lexer.next().expect("Lexer does not contain any tokens!");
        Self {
            lexer: lexer.peekable()
        }
    }
    fn expr(&mut self) -> Result<Expr, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let expr: Expr = self.comparison()?;

        let peeked_token: Option<&Token> = self.lexer.peek();
        match peeked_token {
            Some(t) =>  {
                if t.token_type == TokenType::BangEqual || t.token_type == TokenType::EqualEqual {
                    let expr_right = self.comparison()?;
                    return Ok(
                        Expr::Binary {
                            left: Box::new(expr), op: self.lexer.next().unwrap().token_type, right: Box::new(expr_right)
                        }
                    );
                }
                else {
                    return Ok(expr);
                }
            },
            None => {todo!()}
        }
    }
    fn comparison(&mut self) -> Result<Expr, ()> {
        todo!();
    }
    fn term(&mut self) {}
    fn factor(&mut self) {}
    fn unary(&mut self) {}
    fn primary(&mut self) {}
}

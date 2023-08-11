use crate::scanner::*;
use crate::stmt::*;
use crate::expr::*;

use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    had_error: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Rc<Stmt>>, ()> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            if matches!(self.peek().token_type, TokenType::If | TokenType::While) {
                return;
            }
            self.advance();
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<Token, ()> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            unreachable!("{}", message)
        }
    }

    // ============================================================================
    // 
    // ============================================================================

    fn expression(&mut self) -> Result<Expr, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ()> {
        let expr = self.or()?;

        if self.is_match(&[TokenType::Assignment]) {
            let assign = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(Rc::new(AssignExpr {
                    name: expr.name.clone(),
                    value: Rc::new(value),
                })));
            }
            unreachable!("Invalid assignment target");
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ()> {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ()> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Rc::new(LogicalExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ()> {
        let mut expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ()> {
        let mut expr = self.term()?;

        while self.is_match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ()> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }))
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ()> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Rc::new(BinaryExpr {
                left: Rc::new(expr),
                operator,
                right: Rc::new(right),
            }))
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ()> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(Rc::new(UnaryExpr {
                operator,
                right: Rc::new(right),
            })))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ()> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Bool(false)),
            })));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: Some(Object::Bool(true)),
            })));
        }

        if self.is_match(&[TokenType::NumberLiteral]) {
            return Ok(Expr::Literal(Rc::new(LiteralExpr {
                value: self.previous().literal,
            })));
        }

        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(Rc::new(VariableExpr {
                name: self.previous(),
            })));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Rc::new(GroupingExpr {
                expression: Rc::new(expr),
            })));
        }

        unreachable!("{:?}", self.tokens[self.current])
    }

    fn declaration(&mut self) -> Result<Rc<Stmt>, ()> {
        // TODO: this should check for var declaration first
        self.statement()
    }

    fn statement(&mut self) -> Result<Rc<Stmt>, ()> {
        if self.is_match(&[TokenType::If]) {
            return Ok(Rc::new(self.if_statement()?));
        }

        if self.is_match(&[TokenType::Print]) {
            return Ok(Rc::new(self.print_statement()?));
        }

        if self.is_match(&[TokenType::While]) {
            return Ok(Rc::new(self.while_statement()?));
        }

        if self.is_match(&[TokenType::Do, TokenType::Then]) {
            return Ok(Rc::new(Stmt::Block(Rc::new(BlockStmt {
                statements: Rc::new(self.block()?),
            }))));
        }

        self.expression_statement()
    }

    fn block(&mut self) -> Result<Vec<Rc<Stmt>>, ()> {
        let mut statements = vec![];

        while !self.check(&TokenType::End) && !self.check(&TokenType::Else) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        if !self.check(&TokenType::Else) {
            if self.check(&TokenType::End) {
                self.consume(&TokenType::End, "Expect 'end' after block.")?;
            }

        }
        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Rc<Stmt>, ()> {
        let expr = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Rc::new(Stmt::Expression(Rc::new(ExpressionStmt {
            expression: Rc::new(expr),
        }))))
    }

    fn print_statement(&mut self) -> Result<Stmt, ()> {
        let value = self.expression()?;
        self.consume(&TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Rc::new(PrintStmt {
            expression: Rc::new(value),
        })))
    }

    fn while_statement(&mut self) -> Result<Stmt, ()> {
        let condition = self.expression()?;
        let body = self.statement()?;

        Ok(Stmt::While(Rc::new(WhileStmt {
            condition: Rc::new(condition),
            body,
        })))
    }

    fn if_statement(&mut self) -> Result<Stmt, ()> {
        let condition = self.expression()?;
        let then_branch = self.statement()?;
        let else_branch = if self.is_match(&[TokenType::Else]) {
            Some(self.statement()?)
        } else {
            None
        };

        if self.check(&TokenType::End) {
            self.consume(&TokenType::End, "");
        }

        Ok(Stmt::If(Rc::new(IfStmt {
            condition: Rc::new(condition),
            then_branch,
            else_branch,
        })))
    }
}

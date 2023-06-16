use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

use crate::Object;
use crate::scanner::Token;
use crate::scanner::TokenType;
use crate::stmt::*;
use crate::expr::*;

#[derive(Debug, Clone)]
pub enum ByteCode {
    Push(usize),
    Pop,
    Add,
    Sub,
    Mul,
}

pub struct BytecodeGenerator {
    instructions: Rc<RefCell<Vec<ByteCode>>>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        BytecodeGenerator { instructions: Rc::new(RefCell::new(vec![])) }
    }

    pub fn generate(&mut self, statements: Rc<Vec<Rc<Stmt>>>) -> Vec<ByteCode> {
        for statement in statements.deref() {
            statement.accept(statement.clone(), self);
        }
        self.instructions.borrow().to_vec()
    }
}

impl StmtVisitor<()> for BytecodeGenerator {
    fn visit_block_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &BlockStmt) -> Result<(), ()> {
        todo!()
    }

    fn visit_if_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &IfStmt) -> Result<(), ()> {
        todo!()
    }

    fn visit_expression_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), ()> {
        stmt.expression.accept(stmt.expression.clone(), self)
    }

    fn visit_print_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &PrintStmt) -> Result<(), ()> {
        todo!()
    }

    fn visit_while_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &WhileStmt) -> Result<(), ()> {
        todo!()
    }
}

impl ExprVisitor<()> for BytecodeGenerator {
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<(), ()> {
        todo!()
    }

    fn visit_binary_expr(&self, wrapper: Rc<Expr>, expr: &BinaryExpr) -> Result<(), ()> {
        expr.left.accept(expr.left.clone(), self);
        expr.right.accept(expr.right.clone(), self);
        match &expr.operator {
            Token { token_type: TokenType::Plus, .. } => self.instructions.borrow_mut().push(ByteCode::Add),
            Token { token_type: TokenType::Minus, .. } => self.instructions.borrow_mut().push(ByteCode::Sub),
            Token { token_type: TokenType::Star, .. } => self.instructions.borrow_mut().push(ByteCode::Mul),
            _ => unimplemented!()
        }
        Ok(())
    }

    fn visit_grouping_expr(&self, wrapper: Rc<Expr>, expr: &GroupingExpr) -> Result<(), ()> {
        todo!()
    }

    fn visit_literal_expr(&self, wrapper: Rc<Expr>, expr: &LiteralExpr) -> Result<(), ()> {
        if let Some(value) = &expr.value {
            match value {
                Object::Num(n) => self.instructions.borrow_mut().push(ByteCode::Push(*n as usize)),
                Object::Bool(_) => todo!(),
                Object::Variable(_) => todo!(),
                Object::DivByZeroError => todo!(),
                Object::ArithmeticError => todo!(),
            }
            Ok(())
        } else {
            Err(())
        }
    }

    fn visit_logical_expr(&self, wrapper: Rc<Expr>, expr: &LogicalExpr) -> Result<(), ()> {
        todo!()
    }

    fn visit_unary_expr(&self, wrapper: Rc<Expr>, expr: &UnaryExpr) -> Result<(), ()> {
        todo!()
    }

    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<(), ()> {
        todo!()
    }
}

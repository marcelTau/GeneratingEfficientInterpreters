use crate::scanner::*;
use crate::expr::*;

use std::rc::Rc;

#[derive(Debug)]
pub enum Stmt {
    Block(Rc<BlockStmt>),
    If(Rc<IfStmt>),
    Expression(Rc<ExpressionStmt>),
    Print(Rc<PrintStmt>),
    While(Rc<WhileStmt>),
}

impl Stmt {
    pub fn accept<T>(&self, wrapper: Rc<Stmt>, visitor: &dyn StmtVisitor<T>) -> Result<T, ()> {
        match self {
            Stmt::Block(x) => visitor.visit_block_stmt(wrapper, x),
            Stmt::If(x) => visitor.visit_if_stmt(wrapper, x),
            Stmt::Expression(x) => visitor.visit_expression_stmt(wrapper, x),
            Stmt::Print(x) => visitor.visit_print_stmt(wrapper, x),
            Stmt::While(x) => visitor.visit_while_stmt(wrapper, x),
        }
    }
}

#[derive(Debug)]
pub struct BlockStmt {
    pub statements: Rc<Vec<Rc<Stmt>>>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub condition: Rc<Expr>,
    pub then_branch: Rc<Stmt>,
    pub else_branch: Option<Rc<Stmt>>,
}

#[derive(Debug)]
pub struct ExpressionStmt {
    pub expression: Rc<Expr>,
}

#[derive(Debug)]
pub struct PrintStmt {
    pub expression: Rc<Expr>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub condition: Rc<Expr>,
    pub body: Rc<Stmt>,
}

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&self, wrapper: Rc<Stmt>, stmt: &BlockStmt) -> Result<T, ()>;
    fn visit_if_stmt(&self, wrapper: Rc<Stmt>, stmt: &IfStmt) -> Result<T, ()>;
    fn visit_expression_stmt(&self, wrapper: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<T, ()>;
    fn visit_print_stmt(&self, wrapper: Rc<Stmt>, stmt: &PrintStmt) -> Result<T, ()>;
    fn visit_while_stmt(&self, wrapper: Rc<Stmt>, stmt: &WhileStmt) -> Result<T, ()>;
}


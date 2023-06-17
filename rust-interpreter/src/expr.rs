use crate::scanner::*;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Assign(Rc<AssignExpr>),
    Binary(Rc<BinaryExpr>),
    Grouping(Rc<GroupingExpr>),
    Literal(Rc<LiteralExpr>),
    Logical(Rc<LogicalExpr>),
    Unary(Rc<UnaryExpr>),
    Variable(Rc<VariableExpr>),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, ()> {
        match self {
            Expr::Assign(x) => visitor.visit_assign_expr(x),
            Expr::Binary(x) => visitor.visit_binary_expr(x),
            Expr::Grouping(x) => visitor.visit_grouping_expr(x),
            Expr::Literal(x) => visitor.visit_literal_expr(x),
            Expr::Logical(x) => visitor.visit_logical_expr(x),
            Expr::Unary(x) => visitor.visit_unary_expr(x),
            Expr::Variable(x) => visitor.visit_variable_expr(x),
        }
    }
}

#[derive(Debug)]
pub struct AssignExpr {
    pub name: Token,
    pub value: Rc<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Rc<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: Option<Object>,
}

#[derive(Debug)]
pub struct LogicalExpr {
    pub left: Rc<Expr>,
    pub operator: Token,
    pub right: Rc<Expr>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Rc<Expr>,
}

#[derive(Debug)]
pub struct VariableExpr {
    pub name: Token,
}

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<T, ()>;
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, ()>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, ()>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, ()>;
    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<T, ()>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, ()>;
    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<T, ()>;
}


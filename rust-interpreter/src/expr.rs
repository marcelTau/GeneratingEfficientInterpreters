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
    pub fn accept<T>(&self, wrapper: Rc<Expr>, visitor: &dyn ExprVisitor<T>) -> Result<T, ()> {
        match self {
            Expr::Assign(x) => visitor.visit_assign_expr(wrapper, x),
            Expr::Binary(x) => visitor.visit_binary_expr(wrapper, x),
            Expr::Grouping(x) => visitor.visit_grouping_expr(wrapper, x),
            Expr::Literal(x) => visitor.visit_literal_expr(wrapper, x),
            Expr::Logical(x) => visitor.visit_logical_expr(wrapper, x),
            Expr::Unary(x) => visitor.visit_unary_expr(wrapper, x),
            Expr::Variable(x) => visitor.visit_variable_expr(wrapper, x),
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
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<T, ()>;
    fn visit_binary_expr(&self, wrapper: Rc<Expr>, expr: &BinaryExpr) -> Result<T, ()>;
    fn visit_grouping_expr(&self, wrapper: Rc<Expr>, expr: &GroupingExpr) -> Result<T, ()>;
    fn visit_literal_expr(&self, wrapper: Rc<Expr>, expr: &LiteralExpr) -> Result<T, ()>;
    fn visit_logical_expr(&self, wrapper: Rc<Expr>, expr: &LogicalExpr) -> Result<T, ()>;
    fn visit_unary_expr(&self, wrapper: Rc<Expr>, expr: &UnaryExpr) -> Result<T, ()>;
    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<T, ()>;
}


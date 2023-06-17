use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::expr::*;
use crate::scanner::Token;
use crate::scanner::TokenType;
use crate::stmt::*;
use crate::Object;

#[derive(Debug, Clone)]
pub enum ByteCode {
    Push(usize),
    Pop,
    Add,
    Sub,
    Mul,
    Var(String),
    Eq,
    NEq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

pub struct BytecodeGenerator {
    instructions: Rc<RefCell<Vec<ByteCode>>>,
    variables: Rc<RefCell<HashMap<String, usize>>>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        BytecodeGenerator {
            instructions: Rc::new(RefCell::new(vec![])),
            variables: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn generate(&mut self, statements: Rc<Vec<Rc<Stmt>>>) -> Vec<ByteCode> {
        for statement in statements.deref() {
            statement.accept(statement.clone(), self);
        }
        self.instructions.borrow().to_vec()
    }

    pub fn show_variables(&self) {
        for (k, v) in self.variables.borrow().iter() {
            println!("{k} = {v}");
        }
    }
}

impl StmtVisitor<()> for BytecodeGenerator {
    fn visit_block_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &BlockStmt) -> Result<(), ()> {
        todo!()
    }

    fn visit_if_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &IfStmt) -> Result<(), ()> {
        todo!()
    }

    fn visit_expression_stmt(
        &self,
        wrapper: std::rc::Rc<Stmt>,
        stmt: &ExpressionStmt,
    ) -> Result<(), ()> {
        stmt.expression.accept(stmt.expression.clone(), self)
    }

    fn visit_print_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &PrintStmt) -> Result<(), ()> {
        todo!()
    }

    fn visit_while_stmt(&self, wrapper: std::rc::Rc<Stmt>, stmt: &WhileStmt) -> Result<(), ()> {
        todo!()
    }
}

macro_rules! perform_operation {
    ($self:ident,$insts:ident, $op:tt,$f:ident) => {
        match $insts.as_slice() {
            [.., ByteCode::Push(i), ByteCode::Push(j)] => {
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().push(ByteCode::Push((*i $op *j) as usize));
            },
            [.., ByteCode::Var(name1), ByteCode::Var(name2)] => {
                let i = $self.variables.borrow_mut().get(name1).expect(&format!("variable with '{name1}' is not defined.")).clone();
                let j = $self.variables.borrow_mut().get(name2).expect(&format!("variable with '{name2}' is not defined.")).clone();
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().push(ByteCode::Push((i $op j) as usize));
            },
            [.., ByteCode::Push(i), ByteCode::Var(name2)] => {
                let j = $self.variables.borrow_mut().get(name2).expect(&format!("variable with '{name2}' is not defined.")).clone();
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().push(ByteCode::Push((*i $op j) as usize));
            },
            [.., ByteCode::Var(name1), ByteCode::Push(j)] => {
                let i = $self.variables.borrow_mut().get(name1).expect(&format!("variable with '{name1}' is not defined.")).clone();
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().pop();
                $self.instructions.borrow_mut().push(ByteCode::Push((i $op *j) as usize));
            },
            _ => {
                $self.instructions.borrow_mut().push($f);
            }
        }
    };
}

impl ExprVisitor<()> for BytecodeGenerator {
    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<(), ()> {
        expr.value.accept(expr.value.clone(), self);
        if let Some(Object::Variable(name)) = &expr.name.literal {
            self.instructions.borrow_mut().push(ByteCode::Var(name.to_string()));
            let insts = self.instructions.borrow().clone();
            match insts.as_slice() {
                [.., ByteCode::Push(value), _] => {
                    self.variables.borrow_mut().insert(name.to_string(), *value);
                }
                _ => panic!()
            }
        }
        Ok(())
    }

    fn visit_binary_expr(&self, wrapper: Rc<Expr>, expr: &BinaryExpr) -> Result<(), ()> {
        use ByteCode::*;
        expr.left.accept(expr.left.clone(), self);
        expr.right.accept(expr.right.clone(), self);
        let insts = self.instructions.borrow().clone();
        match &expr.operator {
            Token { token_type: TokenType::Plus, .. } => perform_operation!(self, insts, +, Add),
            Token { token_type: TokenType::Minus, .. } => perform_operation!(self, insts, -, Sub),
            Token { token_type: TokenType::Star, .. } => perform_operation!(self, insts, *, Mul),
            Token { token_type: TokenType::EqualEqual, .. } => perform_operation!(self, insts, ==, Eq),
            Token { token_type: TokenType::LessEqual, .. } => perform_operation!(self, insts, <=, Lte),
            Token { token_type: TokenType::Less, .. } => perform_operation!(self, insts, <, Lt),
            Token { token_type: TokenType::GreaterEqual, .. } => perform_operation!(self, insts, >=, Gte),
            Token { token_type: TokenType::Greater, .. } => perform_operation!(self, insts, >, Gt),
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn visit_grouping_expr(&self, wrapper: Rc<Expr>, expr: &GroupingExpr) -> Result<(), ()> {
        expr.expression.accept(expr.expression.clone(), self)
    }

    fn visit_literal_expr(&self, wrapper: Rc<Expr>, expr: &LiteralExpr) -> Result<(), ()> {
        if let Some(value) = &expr.value {
            match value {
                Object::Num(n) => self
                    .instructions
                    .borrow_mut()
                    .push(ByteCode::Push(*n as usize)),
                Object::Bool(n) => self
                    .instructions
                    .borrow_mut()
                    .push(ByteCode::Push(*n as usize)),
                Object::Variable(name) => self
                    .instructions
                    .borrow_mut()
                    .push(ByteCode::Var(name.to_string())),
                Object::DivByZeroError => todo!(),
                Object::ArithmeticError => todo!(),
            }
            Ok(())
        } else {
            Err(())
        }
    }

    fn visit_logical_expr(&self, wrapper: Rc<Expr>, expr: &LogicalExpr) -> Result<(), ()> {
        expr.left.accept(expr.left.clone(), self);
        expr.right.accept(expr.right.clone(), self);
        match &expr.operator {
            Token {
                token_type: TokenType::And,
                ..
            } => self.instructions.borrow_mut().push(ByteCode::And),
            Token {
                token_type: TokenType::Or,
                ..
            } => self.instructions.borrow_mut().push(ByteCode::Or),
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn visit_unary_expr(&self, wrapper: Rc<Expr>, expr: &UnaryExpr) -> Result<(), ()> {
        todo!()
    }

    fn visit_variable_expr(&self, wrapper: Rc<Expr>, expr: &VariableExpr) -> Result<(), ()> {
        if let Some(Object::Variable(name)) = &expr.name.literal {
            self.instructions
                .borrow_mut()
                .push(ByteCode::Var(name.to_string()));
            Ok(())
        } else {
            Err(())
        }
    }
}

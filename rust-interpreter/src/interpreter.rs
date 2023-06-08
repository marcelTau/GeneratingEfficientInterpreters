use crate::stmt::*;
use crate::expr::*;
use crate::scanner::*;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Interpreter {
    pub globals: Rc<RefCell<HashMap<String, Object>>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn interpret(&self, statements: Rc<Vec<Rc<Stmt>>>) -> bool {
        for statement in statements.deref() {
            if let Err(e) = self.execute(statement.clone()) {
                return false;
            }
        }
        true
    }

    fn evaluate(&self, expr: Rc<Expr>) -> Result<Object, ()> {
        expr.accept(expr.clone(), self)
    }

    fn execute(&self, statement: Rc<Stmt>) -> Result<(), ()> {
        statement.accept(statement.clone(), self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        !matches!(object, Object::Bool(false))
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, _: Rc<Stmt>, stmt: &ExpressionStmt) -> Result<(), ()> {
        self.evaluate(stmt.expression.clone())?;
        Ok(())
    }

    fn visit_if_stmt(&self, _: Rc<Stmt>, stmt: &IfStmt) -> Result<(), ()> {
        if self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.then_branch.clone())
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch.clone())
        } else {
            Ok(())
        }
    }

    fn visit_print_stmt(&self, _: Rc<Stmt>, stmt: &PrintStmt) -> Result<(), ()> {
        let value = self.evaluate(stmt.expression.clone())?;
        println!("{}", value);
        Ok(())
    }

    fn visit_while_stmt(&self, _: Rc<Stmt>, stmt: &WhileStmt) -> Result<(), ()> {
        while self.is_truthy(&self.evaluate(stmt.condition.clone())?) {
            self.execute(stmt.body.clone())?;
        }
        Ok(())
    }

    fn visit_block_stmt(&self, wrapper: Rc<Stmt>, stmt: &BlockStmt) -> Result<(), ()> {
        stmt.statements.iter().try_for_each(|s| self.execute(s.clone()))
    }
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, _: Rc<Expr>, expr: &LiteralExpr) -> Result<Object, ()> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, _: Rc<Expr>, expr: &GroupingExpr) -> Result<Object, ()> {
        self.evaluate(expr.expression.clone())
    }

    fn visit_binary_expr(&self, _: Rc<Expr>, expr: &BinaryExpr) -> Result<Object, ()> {
        let left = self.evaluate(expr.left.clone())?;
        let right = self.evaluate(expr.right.clone())?;

        let result = match expr.operator.token_type {
            TokenType::Star => left * right,
            TokenType::Slash => left / right,
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Greater => Object::Bool(left > right),
            TokenType::GreaterEqual => Object::Bool(left >= right),
            TokenType::Less => Object::Bool(left < right),
            TokenType::LessEqual => Object::Bool(left <= right),
            TokenType::BangEqual => Object::Bool(left != right),
            TokenType::EqualEqual => Object::Bool(left == right),
            _ => unreachable!(),
        };

        match result {
            Object::ArithmeticError | Object::DivByZeroError => panic!("Object::ArithmeticError | Object::DivByZeroError occured"),
            _ => Ok(result),
        }
    }

    fn visit_unary_expr(&self, _: Rc<Expr>, expr: &UnaryExpr) -> Result<Object, ()> {
        let right = self.evaluate(expr.right.clone())?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Object::Num(n) => Ok(Object::Num(n * (-1_f64))),
                _ => unreachable!()
            },
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(&right))),
            _ => unreachable!()
        }
    }

    fn visit_variable_expr(
        &self,
        wrapper: Rc<Expr>,
        expr: &VariableExpr,
    ) -> Result<Object, ()> {
        match &expr.name {
            Token { token_type: TokenType::Identifier, literal: Some(Object::Variable(name)) } => {
                //self.globals.borrow_mut().insert(name.to_string(), value.clone());
                match self.globals.borrow().get(name) {
                    Some(value) => Ok(value.clone()),
                    None => panic!()
                }
            }
            _ => panic!(),
        }
        // self.environment.borrow().borrow().get(&expr.name)
        //self.lookup_variable(&expr.name, wrapper)
    }

    fn visit_assign_expr(&self, wrapper: Rc<Expr>, expr: &AssignExpr) -> Result<Object, ()> {
        let value = self.evaluate(expr.value.clone())?;

        match &expr.name {
            Token { token_type: TokenType::Identifier, literal: Some(Object::Variable(name)) } => {
                self.globals.borrow_mut().insert(name.to_string(), value.clone());
            }
            _ => panic!(),
        }

        //if let Token { token_type, Object::Variable(name) } = &expr.name {
            //self.globals.entry(name).and_modify(value).or_insert(value);
        //}
        //self.globals.entry(&expr.name).and_modify(value).or_insert(value);
        //if let Some(distance) = self.locals.borrow().get(&wrapper) {
            //self.environment
                //.borrow()
                //.borrow_mut()
                //.assign_at(*distance, &expr.name, &value)?;
        //} else {
            //self.globals
                //.borrow_mut()
                //.assign(&expr.name, value.clone())?;
        //}

        Ok(value)
    }

    fn visit_logical_expr(&self, _: Rc<Expr>, expr: &LogicalExpr) -> Result<Object, ()> {
        let left = self.evaluate(expr.left.clone())?;

        if expr.operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if !self.is_truthy(&left) {
            return Ok(left);
        }
        self.evaluate(expr.right.clone())
    }
}

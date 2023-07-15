use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::expr::*;
use crate::scanner::Token;
use crate::scanner::TokenType;
use crate::stmt::*;
use crate::Object;

#[derive(Debug, Clone, PartialEq)]
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
    Jz { label: String, offset: i32 },
    JNz { label: String, offset: i32 },
    Jmp { label: String, offset: i32 },
    Label(String), // Start of a new label
    Print,
    Assign(String),

    /// Superinstructions

    #[cfg(feature = "PushAdd")]
    PushAdd(usize),
    #[cfg(feature = "AssignPushAdd")]
    AssignPushAdd { name: String, value: usize },
}

pub struct BytecodeGenerator {
    instructions: Rc<RefCell<Vec<ByteCode>>>,
    variables: Rc<RefCell<HashMap<String, usize>>>,
    label_counter: Rc<RefCell<usize>>,
}

static mut LABEL_COUNTER: usize = 0;

impl BytecodeGenerator {
    pub fn new() -> Self {
        BytecodeGenerator {
            instructions: Rc::new(RefCell::new(vec![])),
            variables: Rc::new(RefCell::new(HashMap::new())),
            label_counter: Rc::new(RefCell::new(0)),
        }
    }

    pub fn generate(&mut self, statements: Rc<Vec<Rc<Stmt>>>) -> Vec<ByteCode> {
        for statement in statements.deref() {
            statement.accept(self);
        }
        self.instructions.borrow().to_vec()
    }

    pub fn show_variables(&self) {
        for (k, v) in self.variables.borrow().iter() {
            println!("{k} = {v}");
        }
    }

    unsafe fn generate_label(&self, msg: &str) -> String {
        let label = "L_".to_string() + msg + &LABEL_COUNTER.to_string();
        LABEL_COUNTER += 1;
        label
    }
}

impl StmtVisitor<()> for BytecodeGenerator {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), ()> {
        stmt.statements.iter().try_for_each(|s| s.accept(self))
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), ()> {
        stmt.condition.accept(self);

        let if_label = unsafe { self.generate_label("if_label") };
        let else_label = unsafe { self.generate_label("else_label") };
        let end_of_if_label = unsafe { self.generate_label("end_of_if_label") };

        let insts = self.instructions.borrow().clone();

        //match insts.as_slice() {
            //[.., ByteCode::Push(1)] => { // True case
                //self.instructions.borrow_mut().pop();
                //// dont insert label, no jump, just continue executing
                ////self.instructions.borrow_mut().push(ByteCode::JNz { label: label.clone(), offset: 0 });
            //}
            //[.., ByteCode::Push(0)] => { // False case
                //self.instructions.borrow_mut().pop();
                //self.instructions.borrow_mut().push(ByteCode::Jz { label: else_label.clone(), offset: 0 });
            //},
            //_ => unimplemented!()
        //}
        self.instructions.borrow_mut().push(ByteCode::Jz { label: else_label.clone(), offset: 0 });
        stmt.then_branch.accept(self);
        self.instructions.borrow_mut().push(ByteCode::Jmp { label: end_of_if_label.clone(), offset: 0 });
        self.instructions.borrow_mut().push(ByteCode::Label(else_label));

        match &stmt.else_branch {
            Some(branch) => {
                branch.accept(self);
            },
            None => ()
        }
        self.instructions.borrow_mut().push(ByteCode::Label(end_of_if_label));
        Ok(())
    }

    fn visit_expression_stmt(
        &self,
        stmt: &ExpressionStmt,
    ) -> Result<(), ()> {
        stmt.expression.accept(self)
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), ()> {
        stmt.expression.accept(self);
        self.instructions.borrow_mut().push(ByteCode::Print);
        Ok(())
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), ()> {
        let start_label = unsafe { self.generate_label("start_label") };
        let end_label = unsafe { self.generate_label("end_label") };

        self.instructions.borrow_mut().push(ByteCode::Label(start_label.clone()));
        stmt.condition.accept(self);
        self.instructions.borrow_mut().push(ByteCode::Jz { label: end_label.clone(), offset: 0 });
        stmt.body.accept(self);
        self.instructions.borrow_mut().push(ByteCode::Jmp { label: start_label, offset: 0 });
        self.instructions.borrow_mut().push(ByteCode::Label(end_label));
        Ok(())
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
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<(), ()> {
        expr.value.accept(self);
        if let Some(Object::Variable(name)) = &expr.name.literal {
            self.instructions.borrow_mut().push(ByteCode::Assign(name.to_string()));
            //let insts = self.instructions.borrow().clone();
            //println!("{insts:?}");
            //match insts.as_slice() {
                //[.., ByteCode::Push(value), _] => {
                    //self.variables.borrow_mut().insert(name.to_string(), *value);
                //}
                //_ => panic!()
            //}
        }
        Ok(())
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<(), ()> {
        use ByteCode::*;
        expr.left.accept(self);
        expr.right.accept(self);
        let insts = self.instructions.borrow().clone();
        match &expr.operator {
            //Token { token_type: TokenType::Plus, .. } => perform_operation!(self, insts, +, Add),
            //Token { token_type: TokenType::Minus, .. } => perform_operation!(self, insts, -, Sub),
            //Token { token_type: TokenType::Star, .. } => perform_operation!(self, insts, *, Mul),
            Token { token_type: TokenType::Plus, .. } => self.instructions.borrow_mut().push(ByteCode::Add),
            Token { token_type: TokenType::Minus, .. } => self.instructions.borrow_mut().push(ByteCode::Sub),
            Token { token_type: TokenType::Star, .. } => self.instructions.borrow_mut().push(ByteCode::Mul),

            Token { token_type: TokenType::EqualEqual, .. } => self.instructions.borrow_mut().push(ByteCode::Eq),
            Token { token_type: TokenType::BangEqual, .. } => self.instructions.borrow_mut().push(ByteCode::NEq),
            Token { token_type: TokenType::LessEqual, .. } => self.instructions.borrow_mut().push(ByteCode::Lte),
            Token { token_type: TokenType::Less, .. } => self.instructions.borrow_mut().push(ByteCode::Lt),
            Token { token_type: TokenType::GreaterEqual, .. } => self.instructions.borrow_mut().push(ByteCode::Gte),
            Token { token_type: TokenType::Greater, .. } => self.instructions.borrow_mut().push(ByteCode::Gte),
            x => unimplemented!("{:?}", x),
        }
        Ok(())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<(), ()> {
        expr.expression.accept(self)
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<(), ()> {
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

    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<(), ()> {
        expr.left.accept(self);
        expr.right.accept(self);
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

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<(), ()> {
        todo!()
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<(), ()> {
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

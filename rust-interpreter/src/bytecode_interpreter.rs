use std::collections::HashMap;
use crate::ByteCode;

pub struct ByteCodeInterpreter {
    stack: Vec<usize>,
    pc: i32,
    variables: HashMap<String, usize>,
}

impl ByteCodeInterpreter {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            pc: 0,
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, instructions: &[ByteCode]) {
        while self.pc < instructions.len() as i32 {
            let inst = &instructions[self.pc as usize];
            //println!("Stack: {:?}", self.stack);
            //println!("Current: {:?}", inst);
            //println!("pc: {:?}", self.pc);
            match inst {
                ByteCode::PushAdd(value) => {
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a + value);
                }
                ByteCode::Assign(var_name) => {
                    let value = self.stack.pop().unwrap();
                    self.variables.insert(var_name.to_string(), value);
                }
                ByteCode::AssignPushAdd {
                    name: var_name,
                    value: v,
                } => {
                    let a = self.stack.pop().unwrap();
                    self.variables.insert(var_name.to_string(), *v + a);
                }
                ByteCode::Push(value) => {
                    self.stack.push(*value);
                }
                ByteCode::Pop => {
                    self.stack.pop().unwrap();
                }
                ByteCode::Add => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                }
                ByteCode::Sub => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b - a);
                }
                ByteCode::Mul => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b * a);
                }
                ByteCode::Var(name) => {
                    self.stack.push(
                        *self
                            .variables
                            .get(name)
                            .expect(&format!("There is no variable called '{name}'")),
                    );
                }
                ByteCode::Eq => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((a == b) as usize);
                }
                ByteCode::NEq => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((a != b) as usize);
                }
                ByteCode::Lt => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((b < a) as usize);
                }
                ByteCode::Gt => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((b > a) as usize);
                }
                ByteCode::Lte => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((b <= a) as usize);
                }
                ByteCode::Gte => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push((b >= a) as usize);
                }
                ByteCode::And => todo!(),
                ByteCode::Or => todo!(),
                ByteCode::Jz { label, offset } => {
                    if self.stack.pop() == Some(0) {
                        self.pc += *offset;
                    }
                }
                ByteCode::JNz { label, offset } => {
                    if self.stack.pop().unwrap() != 0 {
                        self.pc += *offset;
                    }
                }

                ByteCode::Jmp { label, offset } => {
                    self.pc += *offset;
                }
                ByteCode::Label(_) => (),
                ByteCode::Print => {
                    let value = self.stack.pop().unwrap();
                    println!("{value}");
                }
            }
            self.pc += 1;
        }
    }
}

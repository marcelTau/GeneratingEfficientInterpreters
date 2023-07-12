#![allow(dead_code, non_snake_case, unused)]

mod scanner;
use bytecode::BytecodeGenerator;
use scanner::*;

mod parser;
use parser::*;

mod stmt;

mod expr;

//mod interpreter;
//use interpreter::*;

/*
a := 1
b := 2

if a == b {
    do stuff
} else {
    do other stuff
}

push 1
var a
push 2
var b
cmp a,b // if a == b
jz L1
do if block
jmp L2
L1: do else block
L2: continue other stuff 



 */


struct ByteCodeInterpreter {
    stack: Vec<usize>,
    pc: i32,
    variables: HashMap<String, usize>
}

impl ByteCodeInterpreter {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            pc: 0,
            variables: HashMap::new()
        }
    }

    pub fn interpret(&mut self, instructions: &[ByteCode]) {
        while self.pc < instructions.len() as i32 {
            let inst = &instructions[self.pc as usize];
            //println!("Stack: {:?}", self.stack);
            //println!("Current: {:?}", inst);
            //println!("pc: {:?}", self.pc);
            match inst {
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
                    self.stack.push(*self.variables.get(name).expect(&format!("There is no variable called '{name}'")));
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
                    if *self.stack.last().unwrap() == 0 {
                        self.pc += *offset;
                    }
                }
                ByteCode::JNz { label, offset } => {
                    if *self.stack.last().unwrap() != 0 {
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
                ByteCode::Assign(var_name) => {
                    let value = self.stack.pop().unwrap();
                    self.variables.insert(var_name.to_string(), value);
                }
            }
            self.pc += 1;
        }
    }
}


mod bytecode;
use bytecode::ByteCode;

use std::{rc::Rc, ops::Deref, collections::HashMap};

fn resolve_labels(code: &mut [ByteCode]) {
    let cloned = code.to_owned();
    for (current, inst) in code.iter_mut().enumerate() {
        match inst {
            ByteCode::Jz { label, offset } => {
                *offset = cloned.iter().position(|i| *i == ByteCode::Label(label.to_string())).unwrap_or(0) as i32 - current as i32;
            }
            ByteCode::JNz { label, offset } => {
                *offset = cloned.iter().position(|i| *i == ByteCode::Label(label.to_string())).unwrap_or(0) as i32 - current as i32;
            }
            ByteCode::Jmp { label, offset } => {
                *offset = cloned.iter().position(|i| *i == ByteCode::Label(label.to_string())).unwrap_or(0) as i32 - current as i32;
            }
            _ => ()
        }
    }
}

fn main() -> Result<(), ()> {
    let easy = r#"
        a := 10 + 5; 
        print a; 
        print a * 2;
        print a + 1 * 2;
    "#;

    //let while_code = r#"
        //a := 1;
        //while a < 10 do
            //a := a + 1;
        //end
        //print a;
    //"#;

    //let if_code = r#"
        //a := 10;

        //if a < 10 then
            //print 1;
        //else
            //print 3;
        //end
    //"#;

    //let easy = r#"
        //a := 0;
        //if 10 == 0 then
            //a := 99;
        //else
            //a := 88;
        //end
        //print a;
    //"#;

    //let easy = r#"
        //a := 0;
        //while a < 10 do
            //a := a + 1;
            //print a;
        //end

        //print a;
    //"#;

    let mut scanner = Scanner::new(easy);
    let tokens = scanner.tokenize()?;

    let mut parser = Parser::new(tokens);
    let mut statements = parser.parse()?;

    for s in &statements {
        println!("{s:?}");
    }

    let mut gen = BytecodeGenerator::new();
    let mut insts = gen.generate(Rc::new(statements));

    resolve_labels(&mut insts);

    println!("===== Code =====");
    println!("{easy}");
    println!("===== Variables =====");
    gen.show_variables();
    println!("=====================");

    for inst in insts.iter() {
        println!("{inst:?}");
    }

    println!("===== Output =====");
    let mut bytecode_interpreter = ByteCodeInterpreter::new();
    bytecode_interpreter.interpret(&insts);

    // possible optimization steps

    //let interpreter = Interpreter::new();
    //interpreter.interpret(Rc::new(statements));

    Ok(())
}

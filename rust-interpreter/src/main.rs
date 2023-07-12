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



mod bytecode;
use bytecode::ByteCode;

use std::{rc::Rc, ops::Deref};

fn resolve_labels(code: &mut [ByteCode]) {
    let cloned = code.to_owned();
    for (current, inst) in code.iter_mut().enumerate() {
        match inst {
            ByteCode::Jz { label, offset } => {
                *offset = cloned.iter().position(|i| *i == ByteCode::Label(label.to_string())).unwrap_or(0) - current;
            }
            ByteCode::JNz { label, offset } => {
                *offset = cloned.iter().position(|i| *i == ByteCode::Label(label.to_string())).unwrap_or(0) - current;
            }
            ByteCode::Jmp { label, offset } => {
                *offset = cloned.iter().position(|i| *i == ByteCode::Label(label.to_string())).unwrap_or(0) - current;
            }
            _ => ()
        }
    }
}

fn main() -> Result<(), ()> {
    let code = r#"
        a := 10 + 5; 
        print a; 
        print a * 2;
        print a + 1 * 2;
    "#;

    let while_code = r#"
        a := 1;
        while a < 10 do
            a := a + 1;
        end
        print a;
    "#;

    let if_code = r#"
        a := 10;

        if a < 10 then
            print 1;
        else
            print 3;
        end
    "#;

    let easy = r#"
        a := 0;
        if 10 == 20 then
            a := 99;
        else
            a := 88;
        end
    "#;

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

    for inst in insts {
        println!("{inst:?}");
    }

    // possible optimization steps

    //let interpreter = Interpreter::new();
    //interpreter.interpret(Rc::new(statements));

    Ok(())
}

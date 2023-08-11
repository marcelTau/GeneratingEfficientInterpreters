#![allow(dead_code, non_snake_case, unused)]

mod threaded;
use threaded::ByteCodeInterpreterThreaded;

mod scanner;
use bytecode::BytecodeGenerator;
use scanner::*;

mod parser;
use parser::*;

mod stmt;
mod expr;

mod bytecode_interpreter;
use bytecode_interpreter::*;

mod bytecode;
use bytecode::ByteCode;

use std::{
    collections::HashMap, fs::read_to_string, ops::Deref, path::PathBuf, rc::Rc, time::Instant,
    io::Write,
};

fn resolve_labels(code: &mut [ByteCode]) {
    let cloned = code.to_owned();
    for (current, inst) in code.iter_mut().enumerate() {
        match inst {
            ByteCode::Jz { label, offset } => {
                *offset = cloned
                    .iter()
                    .position(|i| *i == ByteCode::Label(label.to_string()))
                    .unwrap_or(0) as i32
                    - current as i32;
            }
            ByteCode::JNz { label, offset } => {
                *offset = cloned
                    .iter()
                    .position(|i| *i == ByteCode::Label(label.to_string()))
                    .unwrap_or(0) as i32
                    - current as i32;
            }
            ByteCode::Jmp { label, offset } => {
                *offset = cloned
                    .iter()
                    .position(|i| *i == ByteCode::Label(label.to_string()))
                    .unwrap_or(0) as i32
                    - current as i32;
            }
            _ => (),
        }
    }
}

fn run_file(path: std::path::PathBuf) -> Result<(), ()> {
    //println!("===== {} =====", &path.to_str().unwrap());
    let code = read_to_string(&path).expect(&format!("There is no file '{path:?}'"));

    // Start the benchmark (generating)
    let now = Instant::now();

    let mut scanner = Scanner::new(&code);
    let tokens = scanner.tokenize()?;

    let mut parser = Parser::new(tokens);
    let mut statements = parser.parse()?;

    let mut gen = BytecodeGenerator::new();
    let mut insts = gen.generate(Rc::new(statements));

    insert_superinstructions(&mut insts);
    resolve_labels(&mut insts);

    for inst in insts.iter() {
        println!("{inst:?}");
    }

    let elapsed_time = now.elapsed();
    println!("Generating bytecode took {}ms.", elapsed_time.as_millis());

    // Start the benchmark (interpreting)
    let now = Instant::now();

    let mut bytecode_interpreter = ByteCodeInterpreter::new();
    let now = Instant::now();
    bytecode_interpreter.interpret(&insts);
    let elapsed_time = now.elapsed();
    println!("Interpreting took {}ms.", elapsed_time.as_millis());
    //println!("---------------------------------------");
    std::io::stdout().flush();

    // ============================================================================
    // TEST
    // ============================================================================

    // Start the benchmark (interpreting)
    let now = Instant::now();

    let mut bytecode_interpreter = ByteCodeInterpreterThreaded::new(&insts);
    let now = Instant::now();
    bytecode_interpreter.start();
    let elapsed_time = now.elapsed();
    println!("Interpreting (threaded) took {}ms.", elapsed_time.as_millis());
    //println!("---------------------------------------");

    Ok(())
}

fn run_benchmarks() {
    use std::fs;

    let paths = fs::read_dir("./benchmarks").unwrap();

    for path in paths {
        run_file(path.unwrap().path());
    }
}

fn insert_superinstructions(insts: &mut Vec<ByteCode>) {
    let mut i = 1;

    while i < insts.len() - 1 {
        match insts[i] {
            #[cfg(feature = "PushAdd")]
            ByteCode::Add => {
                match insts[i - 1] {
                    ByteCode::Push(value) => {
                        insts[i] = ByteCode::PushAdd(value);
                        insts.remove(i - 1);
                        i -= 1;
                    }
                    _ => ()
                }
            }
            #[cfg(feature = "AssignPushAdd")]
            ByteCode::Assign(ref name) => {
                match insts[i - 1] {
                    #[cfg(feature = "AssignPushAdd")]
                    ByteCode::PushAdd(value) => {
                        insts[i] = ByteCode::AssignPushAdd {
                            name: name.to_string(),
                            value,
                        };
                        insts.remove(i - 1);
                        i -= 1;
                    }
                    #[cfg(feature = "PushAssign")]
                    ByteCode::Push(value) => {
                        insts[i] = ByteCode::PushAssign {
                            name: name.to_string(),
                            value,
                        };
                        insts.remove(i - 1);
                        i -= 1;
                    }
                    _ => ()
                }
            }
            _ => ()
        }
        i += 1;
    }
}

fn main() -> Result<(), ()> {
    let mut args = std::env::args();
    run_file(PathBuf::from(args.nth(1).expect("Pls provide a filename")));

    //run_file(PathBuf::from("./benchmarks/increment_loop_1000000.imp"));
    //run_benchmarks();
    return Ok(());
    //let easy = r#"
    //a := 10 + 5;
    //print a;
    //print a * 2;
    //print a + 1 * 2;
    //"#;

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

    let easy = r#"
        a := 0;
        while a < 10 do
            a := a + 1;
            print a;
        end

        print a;
    "#;

    let mut scanner = Scanner::new(&easy);
    let tokens = scanner.tokenize()?;

    let mut parser = Parser::new(tokens);
    let mut statements = parser.parse()?;

    //for s in &statements {
    //println!("{s:?}");
    //}

    let mut gen = BytecodeGenerator::new();
    let mut insts = gen.generate(Rc::new(statements));

    resolve_labels(&mut insts);

    insert_superinstructions(&mut insts);

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
    Ok(())
}

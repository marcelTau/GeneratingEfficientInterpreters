#![allow(dead_code, non_snake_case, unused)]

mod scanner;
use scanner::*;

mod parser;
use parser::*;

mod stmt;

mod expr;

mod interpreter;
use interpreter::*;

use std::rc::Rc;

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

    let mut scanner = Scanner::new(if_code);
    let tokens = scanner.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    // possible optimization steps

    let interpreter = Interpreter::new();
    interpreter.interpret(Rc::new(statements));

    Ok(())
}

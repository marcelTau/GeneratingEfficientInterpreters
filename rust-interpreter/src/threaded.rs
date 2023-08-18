use std::collections::HashMap;
use std::mem::Discriminant;
use crate::{ByteCode};

pub type Instruction = fn(interp: &mut ByteCodeInterpreterThreaded);

pub struct ByteCodeInterpreterThreaded {
    stack: Vec<usize>,
    pc: i32,
    variables: HashMap<String, usize>,
    ops: HashMap<Discriminant<ByteCode>, Instruction>,
    instructions: Vec<ByteCode>,
}


impl ByteCodeInterpreterThreaded {
    pub fn new(instructions: &[ByteCode]) -> Self {
        let mut interp = Self {
            stack: vec![],
            pc: 0,
            variables: HashMap::new(),
            ops: HashMap::new(),
            instructions: instructions.to_vec(),
        };

        interp.ops.insert(std::mem::discriminant(&ByteCode::Push(0)), Self::op_push);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Pop), Self::op_pop);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Add), Self::op_add);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Sub), Self::op_sub);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Mul), Self::op_mul);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Mod), Self::op_mod);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Var("".to_string())), Self::op_var);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Eq), Self::op_eq);
        interp.ops.insert(std::mem::discriminant(&ByteCode::NEq), Self::op_neq);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Lt), Self::op_lt);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Gt), Self::op_gt);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Lte), Self::op_lte);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Gte), Self::op_gte);
        interp.ops.insert(std::mem::discriminant(&ByteCode::And), Self::op_and);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Or), Self::op_or);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Jz { label: "".to_string(), offset: 0 }), Self::op_jz);
        interp.ops.insert(std::mem::discriminant(&ByteCode::JNz { label: "".to_string(), offset: 0 }), Self::op_jnz);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Jmp { label: "".to_string(), offset: 0 }), Self::op_jmp);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Label("".to_string()) ), Self::op_label);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Print), Self::op_print);
        interp.ops.insert(std::mem::discriminant(&ByteCode::Assign("i".to_string()) ), Self::op_assign);

        #[cfg(feature = "AssignPushAdd")]
        interp.ops.insert(std::mem::discriminant(&ByteCode::AssignPushAdd { name: "".to_string(), value: 0 }), Self::op_assign_push_add);
        #[cfg(feature = "PushAdd")]
        interp.ops.insert(std::mem::discriminant(&ByteCode::PushAdd(0) ), Self::op_push_add);
        #[cfg(feature = "PushAssign")]
        interp.ops.insert(std::mem::discriminant(&ByteCode::PushAssign { name: "".to_string(), value: 0 }), Self::op_push_assign);

        interp
    }

    pub fn start(&mut self) {
        self.ops[&std::mem::discriminant(&self.instructions[self.pc as usize])](self);
    }

    fn next(&mut self) {
        self.pc += 1;
        if self.pc >= self.instructions.len() as i32 {
            return;
        }

        self.ops[&std::mem::discriminant(&self.instructions[self.pc as usize])](self);
    }

    fn op_push(&mut self) {
        if let ByteCode::Push(value) = &self.instructions[self.pc as usize] {
            self.stack.push(*value);
        }
        self.next();
    }

    fn op_pop(&mut self) {
        self.stack.pop().unwrap();
        self.next()
    }

    fn op_add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b + a);

        self.next()
    }

    fn op_sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b - a);
        self.next()
    }

    fn op_mul(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b * a);
        self.next()
    }

    fn op_mod(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(b % a);
        self.next()
    }

    fn op_var(&mut self) {
        if let ByteCode::Var(name) = &self.instructions[self.pc as usize] {
                    self.stack.push(
                        *self
                            .variables
                            .get(name)
                            .expect(&format!("There is no variable called '{name}'")),
                    );
        }
        self.next()
    }

    fn op_eq(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((a == b) as usize);
        self.next()
    }

    fn op_neq(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((a != b) as usize);
        self.next()
    }

    fn op_lt(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((b < a) as usize);
        self.next()
    }

    fn op_gt(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((b > a) as usize);
        self.next()
    }

    fn op_lte(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((b <= a) as usize);
        self.next()
    }

    fn op_gte(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((b >= a) as usize);
        self.next()
    }

    fn op_and(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((b == 1 && a == 1) as usize);
        self.next();
    }

    fn op_or(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push((b == 1 || a == 1) as usize);
        self.next();
    }

    fn op_jz(&mut self) {
        if let ByteCode::Jz { label, offset } = &self.instructions[self.pc as usize] {
            if self.stack.pop() == Some(0) {
                self.pc += *offset;
            }
        }
        self.next();
        //self.ops[&std::mem::discriminant(&self.instructions[self.pc as usize])](self);
    }

    fn op_jnz(&mut self) {
        if let ByteCode::JNz { label, offset } = &self.instructions[self.pc as usize] {
            if self.stack.pop() != Some(0) {
                self.pc += *offset;
            }
        }
        self.next();
        //self.ops[&std::mem::discriminant(&self.instructions[self.pc as usize])](self);
    }

    fn op_jmp(&mut self) {
        if let ByteCode::Jmp { label, offset } = &self.instructions[self.pc as usize] {
            self.pc += *offset;
        }
        self.next();
        //self.ops[&std::mem::discriminant(&self.instructions[self.pc as usize])](self);
    }

    fn op_print(&mut self) {
        let value = self.stack.pop().unwrap();
        println!("{value}");
        self.next()
    }

    fn op_label(&mut self) {
        self.next()
    }

    fn op_assign(&mut self) {
        if let ByteCode::Assign(var_name) = &self.instructions[self.pc as usize] {
            let value = self.stack.pop().unwrap();
            self.variables.insert(var_name.to_string(), value);
        }
        self.next();
    }


    #[cfg(feature = "PushAdd")]
    fn op_push_add(&mut self) {
        if let ByteCode::PushAdd(value) = &self.instructions[self.pc as usize] {
            let a = self.stack.pop().unwrap();
            self.stack.push(a + *value);
        }
        self.next();
    }

    #[cfg(feature = "AssignPushAdd")]
    fn op_assign_push_add(&mut self) {
        if let ByteCode::AssignPushAdd { name, value } = &self.instructions[self.pc as usize] {
            let x = self.stack.pop().unwrap();
            self.variables.insert(name.to_string(), value + x);
        }
        self.next();
    }
    #[cfg(feature = "PushAssign")]
    fn op_push_assign(&mut self) {
        if let ByteCode::PushAssign { name, value } = &self.instructions[self.pc as usize] {
            self.variables.insert(name.to_string(), *value);
        }
        self.next();
    }
}

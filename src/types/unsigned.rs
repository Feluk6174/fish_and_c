use std::fs::File;
use std::io::Write;

use crate::compiler::{functions::{is_function, Signature}, register::Register, variables::{is_variable, Variable, Variables}, operation::operate};
use crate::precompile::branch::Branch;


pub fn load_unsigned(dest_reg: &Register, var: &Branch, vars: &Variables, signatures: &Vec<Signature>, file: &mut File) -> Result<(), String>{
    if is_variable(vars, &var.token.text) {
        load_unsigned_var(dest_reg, &var.token.text, vars, file)?;
        Ok(())
    }
    else if is_function(signatures, &var.token.text) {
        load_unsigned_func(dest_reg, var, vars, signatures, file);
        Ok(())
    }
    else {
        Err(String::from("Unexpected next token lol"))
    }
}

fn load_unsigned_var(dest_reg: &Register, name: &str, vars: &Variables, file: &mut File) -> Result<(), String> {
    let var = vars.get(name)?;
    
    file.write_all(format!("lea rsi, [r15+{}]
mov {}, [rsi]
", var.rel_pos, dest_reg.name).as_bytes()).expect("Error writing to file");

    Ok(())
}

fn load_unsigned_func(dest_Reg: &Register, function: &Branch, vars: &Variables, signatures: &Vec<Signature>, file: &mut File) {}

pub fn add(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, file: &mut File) {
    file.write_all(format!("add {}, {}\nmov {}, {}\n", op_reg_1.name, op_reg_2.name, dest_reg.name, op_reg_1.name).as_bytes()).expect("Error writing to file");
}

pub fn sub(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, file: &mut File) {
    file.write_all(format!("sub {}, {}\nmov {}, {}\n", op_reg_1.name, op_reg_2.name, dest_reg.name, op_reg_1.name).as_bytes()).expect("Error writing to file");
}

pub fn mul(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, file: &mut File) {
    let temp = Register::new_gen("a", dest_reg.size).unwrap();
    file.write_all(format!("mov {}, {}
mul {}
mov {}, {}
", temp.name, op_reg_1.name, op_reg_2.name, dest_reg.name, temp.name).as_bytes()).expect("Error writing to file");
}

pub fn div(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, file: &mut File) {
    let temp = Register::new_gen("a", dest_reg.size).unwrap();
    file.write_all(format!("mov {}, {}
mov rdx, 0
div {}
mov {}, {}
", temp.name, op_reg_1.name, op_reg_2.name, dest_reg.name, temp.name).as_bytes()).expect("Error writing to file");
}


pub fn modul() {}

pub fn equals(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, comp_idx: u64, file: &mut File) {
    file.write_all(format!("cmp {}, {}
je comp{}
mov {}, 0
jmp ncomp{}
comp{}:
mov {}, 1
ncomp{}:
", op_reg_1.name, op_reg_2.name, comp_idx, dest_reg.name, comp_idx, comp_idx, dest_reg.name, comp_idx).as_bytes()).expect("Error writing to file");
}

pub fn nequals(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, comp_idx: u64, file: &mut File) {
    file.write_all(format!("cmp {}, {}
jne comp{}
mov {}, 0
jmp ncomp{}
comp{}:
mov {}, 1
ncomp{}:
", op_reg_1.name, op_reg_2.name, comp_idx, dest_reg.name, comp_idx, comp_idx, dest_reg.name, comp_idx).as_bytes()).expect("Error writing to file");
}

pub fn grater(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, comp_idx: u64, file: &mut File) {
    file.write_all(format!("cmp {}, {}
jg comp{}
mov {}, 0
jmp ncomp{}
comp{}:
mov {}, 1
ncomp{}:
", op_reg_1.name, op_reg_2.name, comp_idx, dest_reg.name, comp_idx, comp_idx, dest_reg.name, comp_idx).as_bytes()).expect("Error writing to file");
}

pub fn lesser(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, comp_idx: u64, file: &mut File) {
    file.write_all(format!("cmp {}, {}
jl comp{}
mov {}, 0
jmp ncomp{}
comp{}:
mov {}, 1
ncomp{}:
", op_reg_1.name, op_reg_2.name, comp_idx, dest_reg.name, comp_idx, comp_idx, dest_reg.name, comp_idx).as_bytes()).expect("Error writing to file");
}

pub fn gratere(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, comp_idx: u64, file: &mut File) {
    file.write_all(format!("cmp {}, {}
jge comp{}
mov {}, 0
jmp ncomp{}
comp{}:
mov {}, 1
ncomp{}:
", op_reg_1.name, op_reg_2.name, comp_idx, dest_reg.name, comp_idx, comp_idx, dest_reg.name, comp_idx).as_bytes()).expect("Error writing to file");
}

pub fn lessere(dest_reg: &Register, op_reg_1: &Register, op_reg_2: &Register, comp_idx: u64, file: &mut File) {
    file.write_all(format!("cmp {}, {}
jle comp{}
mov {}, 0
jmp ncomp{}
comp{}:
mov {}, 1
ncomp{}:
", op_reg_1.name, op_reg_2.name, comp_idx, dest_reg.name, comp_idx, comp_idx, dest_reg.name, comp_idx).as_bytes()).expect("Error writing to file");
}


pub fn load_pointer_name(vars: &Variables, branch: &Branch, register: &Register, file: &mut File) -> Result<(), String> {
    if !is_variable(vars, &branch.branches[0].token.text) {
        return Err(format!("Undeclared variable {}", branch.branches[0].token.text))
    }
    let var = vars.get(&branch.branches[0].token.text)?;
    file.write_all(format!("lea rsi, [r15+{}]
mov rdi, [rsi]
lea rsi, [rdi]
mov {}, [rsi]
", var.rel_pos, register.name).as_bytes()).expect("Error writing to file");
    
    Ok(())
}



pub fn load_pointer_op(name:&str, vars: &mut Variables, signatures: &Vec<Signature>, branch: &Branch, store_reg: &Register, assist_reg_1: &Register, assist_reg_2: &Register, comp_idx: &mut u64, file: &mut File) -> Result<(), String> {
    operate(name, &branch.branches[0].branches, 0, branch.branches[0].branches.len(), vars, signatures, &Register::new_gen("si", 8)?, &assist_reg_1.get_in_size(8)?, &assist_reg_2.get_in_size(8)?, comp_idx, file)?;
    
    file.write_all(format!("mov {}, {}[rsi]
", store_reg.name, store_reg.prefix()).as_bytes()).expect("Error writing to file");
    Ok(())
}
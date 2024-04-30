use std::fs::File;
use std::io::Write;

use crate::compiler::{register::Register, variables::{Variables, Variable}, functions::Signature};
use crate::precompile::{branch::Branch, tokens::TTS};


pub fn load_unsigned(dest_reg: &Register, var: &Branch, vars: &Variables, signatures: &Vec<Signature>, file: &mut File) -> Result<(), String>{
    if var.branches.len() == 0 {
        load_unsigned_var(dest_reg, &var.token.text, vars, file)?;
        Ok(())
    }
    else if var.branches[0].token.token_type == TTS::Parenthesis && var.branches[0].token.text == "(" {
        load_unsigned_func(dest_reg, var, vars, signatures, file);
        Ok(())
    }
    else {
        Err(String::from("Unexpected next token lol"))
    }
}

fn load_unsigned_var(dest_reg: &Register, name: &str, vars: &Variables, file: &mut File) -> Result<(), String> {
    let var = vars.get(name)?;
    
    file.write_all(format!("lea rsi, [mem+{}]
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

pub fn div() {}

pub fn modul() {}
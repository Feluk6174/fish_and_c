use std::fs::File;
use std::io::Write;
use crate::compiler::register::Register;
use crate::compiler::variables::{PointerType, Type, Variable, Variables};
use crate::precompile::branch::Branch;
use crate::precompile::tokens::TTS;

pub fn load_number_literal(value: &str, register: &Register, file:&mut File) {
    file.write_all(format!("mov {}, {}\n", register.name, value).as_bytes()).expect("Error writing to file");
}

pub fn load_register_result(start_reg: &str, dest_reg: &Register, file:&mut File) {
    if start_reg == dest_reg.name {
        return 
    }
    file.write_all(format!("mov {}, {}\n", dest_reg.name, start_reg).as_bytes()).expect("Error writing to file");
}

pub fn declare_string_literal(name: &str, vars: &mut Variables, content:&str, file: &mut File) -> Result<(), String> {
    vars.push(Variable::new_direct(name, Type::Pointer(PointerType::U8), vars.rel_pos)?);
    let var = vars.get(name)?;
    let mut asm = format!("lea rdi, [r15+{}]\nlea rax, [rdi+8]\nmov QWORD[rdi], rax\n", var.rel_pos);
    asm += &format!("lea rsi, [r15+{}]\n", vars.rel_pos);
    for ch in content.chars() {
        asm += &format!("mov al, '{}'\nmov BYTE[rsi], al\nadd rsi, 1\n", ch, );
    }
    asm += "mov al, 0\nmov BYTE[rsi], al\nadd rsi, 1\n";
    file.write_all(asm.as_bytes()).expect("Failed to write to file");
    vars.rel_pos += content.len() as u64+1;
    Ok(())
}
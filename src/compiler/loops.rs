use std::{fs::File, io::Write};

use crate::precompile::branch::Branch;

use super::{functions::{Function, Signature}, operation::operate, register::Register};




pub fn gen_while_asm(branch: &Branch, signatures: &Vec<Signature>, function: &mut Function, file: &mut File) -> Result<(), String> {

    function.loops.1 += 1;
    function.loops.0.push(function.loops.1);

    file.write_all(format!("do{}:\n", function.loops.1).as_bytes()).expect("Failed to write to file");
    
    operate("None", &branch.branches, 0, branch.branches.len() - 1, &mut function.vars, signatures, &Register::new_gen("a", 1)?, &Register::new_gen("b", 1)?, &Register::new_gen("c", 1)?, &mut function.comp_idx, file)?;
    
    file.write_all(format!("cmp al, 0
je done{}\n", function.loops.1).as_bytes()).expect("Failed to write to file");

    function.process_custom(file, signatures, &branch.branches[branch.branches.len() - 1])?;

    let tnum = function.loops.0.pop().unwrap();
    file.write_all(format!("jmp do{}\ndone{}:\n", tnum, tnum).as_bytes()).expect("Failed to write to file");
    Ok(())
}

pub fn gen_break_asm(loops: &(Vec<u64>, u64), file: &mut File) -> Result<(), String> {
    file.write_all(format!("jmp done{}\n", loops.0[loops.0.len()-1]).as_bytes()).expect("Failed to write to file");
    Ok(())
}

pub fn gen_continue_asm(loops: &(Vec<u64>, u64), file: &mut File) -> Result<(), String> {
    file.write_all(format!("jmp do{}\n", loops.0[loops.0.len()-1]).as_bytes()).expect("Failed to write to file");
    Ok(())
}
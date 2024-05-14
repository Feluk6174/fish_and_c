use std::{fs::File, io::Write};

use crate::precompile::{branch::Branch, tokens::TTS};

use super::{functions::{Function, Signature}, operation::operate, register::Register};

pub fn gen_if_asm(branch: &Branch, signatures: &Vec<Signature>, function: &mut Function, file: &mut File) -> Result<(), String> {
    let has_elese = branch.branches[branch.branches.len()-1].token.token_type == TTS::ElseKeyword;
    let extra = match has_elese {
        true => 2,
        false => 1
    };
    function.ifs.1 += 1;
    function.ifs.0.push(function.ifs.1);

    operate("None", &branch.branches, 0, branch.branches.len() - extra, &mut function.vars, signatures, &Register::new_gen("a", 1)?, &Register::new_gen("b", 1)?, &Register::new_gen("c", 1)?, &mut function.comp_idx, file)?;
    
    file.write_all(format!("cmp al, 0
je else{}\n", function.ifs.1).as_bytes()).expect("Failed to write to file");

    function.process_custom(file, signatures, &branch.branches[branch.branches.len() - extra])?;

    file.write_all(format!("jmp fi{}\nelse{}:\n", function.ifs.1, function.ifs.1).as_bytes()).expect("Failed to write to file");

    if has_elese {
        let last = branch.branches.len() - 1;
        let else_branch = &branch.branches[last];
        if else_branch.branches[0].token.token_type == TTS::Keys {
            function.process_custom(file, signatures, &else_branch.branches[0])?;
        } else if else_branch.branches[0].token.token_type == TTS::IfKeyword {
            gen_if_asm(&else_branch.branches[0], signatures, function, file)?;
        }
    }

    file.write_all(format!("fi{}:\n", function.ifs.0.pop().unwrap()).as_bytes()).expect("Failed to write to file");
    Ok(())
}
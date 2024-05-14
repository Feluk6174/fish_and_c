use std::fs::File;
use std::io::Write;

use crate::compiler::functions::Signature;
use crate::compiler::operation::operate;
use crate::compiler::register::Register;
use crate::compiler::variables::{Variables, Size, Type};
use crate::precompile::{branch::Branch, tokens::TTS};

pub static BUILTIN_FUNCTIONS: [&str; 3] = ["resmem", "sizeof", "reg"];

pub fn builtin(name: &str, branch: &Branch, vars: &mut Variables, signatures: &Vec<Signature>, register: &Register, comp_idx: &mut u64, file: &mut File) -> Result<bool, String> {
    match name {
        "resmem" => {
            resmem(vars, branch, register, file)?;
            Ok(true)
        },
        "sizeof" => {
            sizeof(vars, branch, register, file)?;
            Ok(true)
        },
        "reg" =>   {
            reg(vars, signatures, branch, comp_idx, file)?;
            Ok(true)
        }
        _ => Ok(false)
    }

}

fn resmem(vars: &mut Variables, branch: &Branch, register: &Register, file: &mut File) -> Result<(), String> {
    if branch.branches[0].branches[0].token.token_type != TTS::NumberLiteral {
        return Err(String::from("resmem takes only number literals"))
    }

    file.write_all(format!("lea {}, [r15+{}]\n", register.name, vars.rel_pos).as_bytes()).expect("Failed to write to file");

    vars.rel_pos += match branch.branches[0].branches[0].token.text.parse::<u64>() {
        Ok(num) => num,
        Err(_) => return Err(format!("Couldn convert {} to number", branch.branches[0].branches[0].token.text))
    };

    Ok(())
}

fn sizeof(vars: &Variables, branch: &Branch, register: &Register, file: &mut File) -> Result<(), String> {
    let branch = &branch.branches[0].branches[0];
    let size: u64;
    match branch.token.token_type {
        TTS::VarType => {
            size = Type::new(branch)?.pure_size()?;
        },
        TTS::Name => {
            size = vars.get(&branch.token.text)?.pure_size;

        },
        TTS::Pointer => {
            size = Type::new(branch)?.pure_size()?;
        },
        _ => return Err(String::from("sizeof only takes types, variables or pointers")),
    }

    file.write(format!("mov {}, {}", register.name, size).as_bytes()).expect("Failed to write to file");

    Ok(())
}

fn reg(vars: &mut Variables, signatures: &Vec<Signature>, branch: &Branch, comp_idx: &mut u64, file: &mut File) -> Result<(), String> {
    if branch.branches[0].branches[0].token.token_type != TTS::StringLiteral {
        return Err("Expected string literal as first argument for reg".to_string())
    }
    if branch.branches[1].branches[0].token.token_type != TTS::NumberLiteral {
        return Err("Expected number literal as first argument for reg".to_string())
    }
    let size = match branch.branches[1].branches[0].token.text.parse::<u64>() {
        Ok(val) => val,
        Err(_) => return Err(format!("Expected a number got {}", branch.branches[1].branches[0].token.text))
    };
    operate("None", &branch.branches[2].branches, 0, branch.branches[2].branches.len(), vars, signatures, &Register::new(&branch.branches[0].branches[0].token.text, "a", size), &Register::new_gen("13", size)?, &Register::new_gen("14", size)?, comp_idx, file)?;
    Ok(())
}
use std::collections::HashMap;
use std::fs::File;
use crate::precompile::tokens::TTS;
use crate::precompile::branch::{Branch, get_name_from_arg};
use super::assembly::gen_asm_asm;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Branch,
    pub code: Branch,
    pub args: Vec<Branch>,
    pub vars: HashMap<String, Branch>
}

impl Function {
    pub fn new(tree: &Vec<Branch>, idx:usize) -> Result<Self, String> {
        if tree[idx].token.token_type != TTS::Function {
            return Err(String::from("Expected function"))
        }

        if tree[idx + 1].token.token_type != TTS::Keys {
            return Err(String::from("Expected code block"))
        }
        
        let mut args = Vec::new();
        let mut vars = HashMap::new();
        for i in 0..tree[idx].branches[2].branches.len() {
            args.push(tree[idx].branches[2].branches[i].clone());
            vars.insert(get_name_from_arg(&tree[idx].branches[2].branches[i])?, (tree[idx].branches[2].branches[i].clone()));
        }


        println!("{:?}", vars.keys());

        Ok(Self {
            name: tree[idx].branches[1].token.text.clone(),
            return_type: tree[idx].branches[0].clone(),
            code: tree[idx+1].clone(),
            args: args,
            vars: vars
        })
    }

    pub fn process(mut self, mut file:File) -> Result<(), String>{
        for branch in &mut self.code.branches[0].branches {
            match branch.token.token_type {
                TTS::Pointer | TTS::VarType => {}
                TTS::Name => {}
                TTS::IfKeyword => {}
                TTS::WhileKeyword => {}
                TTS::ReturnKeyword => {}
                TTS::BreakKeyword => {}
                TTS::ContinueKeyword => {}
                TTS::Assembly => gen_asm_asm(branch, &mut file)?,
                _ => return Err(format!("Invalid token {}", branch.token.text))
            };
        }
        Ok(())
    }
}


pub fn build_functions(tree: &Vec<Branch>) -> Result<Vec<Function>, String> {
    let mut functions: Vec<Function> = Vec::new();
    for i in 0..tree.len() / 2 {
        functions.push(Function::new(tree, i*2)?);
    }
    Ok(functions)
}

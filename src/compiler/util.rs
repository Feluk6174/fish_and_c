use std::collections::HashMap;
use crate::precompile::tokens::{Token, TTS};
use crate::precompile::branch::Branch;

pub struct Function {
    return_type: Branch,
    code: Branch,
    args: Vec<Branch>,
    vars: HashMap<String, (Branch)>
}

impl Function {
    pub fn new(tree: Vec<Branch>, idx:usize) -> Result<Self, String> {
        if tree[idx].token.token_type != TTS::Function {
            return Err(String::from("Expected function"))
        }

        if tree[idx + 1].token.token_type != TTS::Keys {
            return Err(String::from("Expected code block"))
        }
        
        let mut args = Vec::new();
        let mut vars = HashMap::new();
        for i in 1..tree[idx].branches.len() {
            args.push(tree[idx].branches[i].clone());
            //vars.insert(tree[idx].branches[i]., v)
        }


        Ok(Self {
            return_type: tree[idx].branches[0].clone(),
            code: tree[idx+1].clone(),
            args: args,
            vars: vars
        })
    }
}


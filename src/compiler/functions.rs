use super::assembly::gen_asm_asm;
use super::variables::{Variables, Type};
use crate::compiler::variables::gen_declare_asm;
use crate::precompile::branch::{get_name_from_arg, Branch};
use crate::precompile::tokens::TTS;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct Signature {
    pub name: String,
    pub return_type: Type,
    pub args: Vec<Type>
}

impl Signature {
    pub fn new(function: &Function) -> Self {
        Self {
            name: function.name.clone(),
            return_type: function.return_type,
            args: function.args.clone()
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub code: Branch,
    pub args: Vec<Type>,
    pub vars: Variables,
}

impl Function {
    pub fn new(tree: &Vec<Branch>, idx: usize) -> Result<Self, String> {
        if tree[idx].token.token_type != TTS::Function {
            return Err(String::from("Expected function"));
        }

        if tree[idx + 1].token.token_type != TTS::Keys {
            return Err(String::from("Expected code block"));
        }

        let mut args = Vec::new();
        let mut vars = Variables::new();
        for i in 0..tree[idx].branches[2].branches.len() {
            args.push(Type::new(&tree[idx].branches[2].branches[i])?);
            vars.push_arg(get_name_from_arg(&tree[idx].branches[2].branches[i])?, &tree[idx].branches[2].branches[i])?;
        }

        println!("{:?} {}", vars.names(), vars.rel_pos);

        Ok(Self {
            name: tree[idx].branches[1].token.text.clone(),
            return_type: Type::new(&tree[idx].branches[0])?,
            code: tree[idx + 1].clone(),
            args: args,
            vars: vars,
        })
    }

    pub fn process(&mut self, file: &mut File, mut signatures: &Vec<Signature>) -> Result<(), String> {
        self.add_start(file);
        for branch in &self.code.branches {
            // println!("{:?}", branch.token);
            match branch.token.token_type {
                TTS::Pointer | TTS::VarType => gen_declare_asm(&mut self.vars, signatures, branch, file)?,
                TTS::Name => {}
                TTS::IfKeyword => {}
                TTS::WhileKeyword => {}
                TTS::ReturnKeyword => {}
                TTS::BreakKeyword => {}
                TTS::ContinueKeyword => {}
                TTS::Assembly => gen_asm_asm(&branch, file)?,
                _ => return Err(format!("Invalid token {}", branch.token.text)),
            };
        }
        self.add_end(file);
        Ok(())
    }

    fn add_end(&mut self, file: &mut File) {
        file.write_all("pop rbp\nret\n".as_bytes()).expect("Failed tor write to file!")
    }

    fn add_start(&mut self, file: &mut File) {
        file.write_all(format!("{}:
    push rbp
    mov rbp, rsp
", self.name).as_bytes())
            .expect("Failed tor write to file!")
    }
}

pub fn process_functions(mut functions: Vec<Function>, signatures: Vec<Signature>, file: &mut File) -> Result<(), String> {
    for i in 0..functions.len() {
        functions[i].process(file, &signatures)?;
    }
    Ok(())
}

pub fn build_functions(tree: &Vec<Branch>) -> Result<(Vec<Function>, Vec<Signature>), String> {
    let mut functions: Vec<Function> = Vec::new();
    let mut signatures:Vec<Signature> = Vec::new();
    for i in 0..tree.len() / 2 {
        functions.push(Function::new(tree, i * 2)?);
        signatures.push(Signature::new(&functions[i]));
    }
    Ok((functions, signatures))
}

pub fn call() {}
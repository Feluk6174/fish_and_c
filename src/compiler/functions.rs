use super::assembly::gen_asm_asm;
use super::comparison::gen_if_asm;
use super::operation::operate;
use super::register::Register;
use super::variables::{assignate_var, is_variable, Size, Type, Variables};
use crate::compiler::variables::gen_declare_asm;
use crate::precompile::branch::{get_name_from_arg, Branch};
use crate::precompile::tokens::TTS;
use crate::runtime::functions::{builtin, builtin_functions};
use std::fs::File;
use std::io::Write;
use std::iter::zip;

#[derive(Debug)]
pub struct Signature {
    pub name: String,
    pub return_type: Type,
    pub args: Vec<Type>,
}

impl Signature {
    pub fn new(function: &Function) -> Self {
        Self {
            name: function.name.clone(),
            return_type: function.return_type,
            args: function.args.clone(),
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
    pub ifs: (Vec<u64>, u64)
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
            vars.push_arg(
                get_name_from_arg(&tree[idx].branches[2].branches[i])?,
                &tree[idx].branches[2].branches[i],
            )?;
        }


        Ok(Self {
            name: tree[idx].branches[1].token.text.clone(),
            return_type: Type::new(&tree[idx].branches[0])?,
            code: tree[idx + 1].clone(),
            args: args,
            vars: vars,
            ifs: (Vec::new(), 0)
        })
    }

    pub fn process(
        &mut self,
        file: &mut File,
        mut signatures: &Vec<Signature>,
    ) -> Result<(), String> {
        self.add_start(file);
        self.process_custom(file, signatures,&self.code.clone())?;
        self.add_end(file);
        Ok(())
    }

    pub fn process_custom(
        &mut self,
        file: &mut File,
        mut signatures: &Vec<Signature>,
        branch: &Branch
    ) -> Result<(), String> {
        for branch in &branch.branches {
            match branch.token.token_type {
                TTS::Pointer | TTS::VarType => {
                    gen_declare_asm(&mut self.vars, signatures, branch, file)?
                }
                TTS::Name => if is_function(signatures, &branch.token.text) {
                    function_call(&branch.token.text, signatures, &mut self.vars, file, branch)?;
                } else if is_variable(&self.vars, &branch.token.text) {
                    assignate_var(&branch.token.text, &mut self.vars, signatures, branch, file)?
                } else {
                    return Err(format!("Undeclared name {}!", branch.token.text))
                },
                TTS::IfKeyword => {
                    gen_if_asm(&branch, signatures, self, file)?
                },
                TTS::WhileKeyword => {}
                TTS::ReturnKeyword => {}
                TTS::BreakKeyword => {}
                TTS::ContinueKeyword => {}
                TTS::Assembly => gen_asm_asm(&branch, file)?,
                _ => return Err(format!("Invalid token {}", branch.token.text)),
            };
        }
        Ok(())
    }

    fn add_end(&mut self, file: &mut File) {
        file.write_all("ret\n".as_bytes())
            .expect("Failed tor write to file!")
    }

    fn add_start(&mut self, file: &mut File) {
        file.write_all(
            format!(
                "{}:
",
                self.name
            )
            .as_bytes(),
        )
        .expect("Failed tor write to file!")
    }
}

pub fn process_functions(
    mut functions: Vec<Function>,
    signatures: Vec<Signature>,
    file: &mut File,
) -> Result<(), String> {
    for i in 0..functions.len() {
        functions[i].process(file, &signatures)?;
    }
    Ok(())
}

pub fn build_functions(tree: &Vec<Branch>) -> Result<(Vec<Function>, Vec<Signature>), String> {
    let mut functions: Vec<Function> = Vec::new();
    let mut signatures: Vec<Signature> = Vec::new();
    for i in 0..tree.len() / 2 {
        functions.push(Function::new(tree, i * 2)?);
        signatures.push(Signature::new(&functions[i]));
    }
    Ok((functions, signatures))
}

pub fn is_function(signatures: &Vec<Signature>, name: &str) -> bool {
    for signature in signatures {
        if &signature.name == name {
            return true;
        }
    }

    for fname in builtin_functions {
        if fname == name {
            return true
        }
    }

    false
}

fn get_sign<'a>(name: &str, signatures: &'a Vec<Signature>) -> Result<&'a Signature, String>{
    for signature in signatures {
        if signature.name == name {
            return Ok(signature)
        }
    }
    Err(format!("Function {} not found", name))
}

pub fn function_call(name:&str, signatures: &Vec<Signature>, vars: &mut Variables, file: &mut File, branch: &Branch) -> Result<Register, String> {
    if builtin(name, branch, vars, signatures, &Register::new_gen("8", 8)?, file)? {
        return Ok(Register::new_gen("8", 8)?)
    }
    let signature = get_sign(name, signatures)?;
    let mut rel_pos:u64  = vars.rel_pos;
    for (t, arg) in zip(&signature.args,&branch.branches) {
        let reg = Register::new_gen("8", t.pure_size()?)?;

        operate("None", &arg.branches, 0, 1, vars, signatures, &reg, &Register::new_gen("c", t.pure_size()?)?, &Register::new_gen("d", t.pure_size()?)?, file)?;
        file.write_all(format!("lea rsi, [r15+{}]
mov {}[rsi], {}\n", rel_pos, reg.prefix(), reg.name).as_bytes()).expect("Failed to write to file");
        rel_pos += t.pure_size()?;

    }
    file.write_all(format!("push r15
add r15, {}
call {}
pop r15
", vars.rel_pos, name).as_bytes()).expect("Failed to write to file");

    Ok(Register::new_gen("8", signature.return_type.pure_size()?)?)
}

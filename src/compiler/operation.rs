use super::{
    functions::{function_call, is_function, Signature},
    register::{store_reg_to_mem, Register},
    variables::{load_address, Variables},
};
use crate::types::{
    literals::{load_number_literal, load_register_result},
    unsigned::{add, div, load_pointer_op, load_unsigned, mul, sub},
};
use crate::{
    precompile::{
        branch::Branch,
        tokens::{Token, TTS},
    },
    types::unsigned::load_pointer_name,
};
use std::{fs::File, io::Write};

fn is_operation(token: &Token) -> bool {
    token.token_type == TTS::ArithmeticOperation || token.token_type == TTS::Comparison
}

fn get_priority(token: &Token) -> Result<u8, String> {
    if token.token_type == TTS::ArithmeticOperation {
        if token.text == "+" || token.text == "-" {
            Ok(1)
        } else if token.text == "*" || token.text == "/" || token.text == "%" {
            Ok(2)
        } else if token.text == "end" {
            Ok(0)
        } else {
            Err(format!("{} is not an arithmetic operation", token.text))
        }
    } else {
        Ok(4)
    }
}

fn load_branch(
    num: &Branch,
    assist_reg: &Register,
    vars: &Variables,
    functions: &Vec<Signature>,
    file: &mut File,
) -> Result<(), String> {
    match num.token.token_type {
        TTS::NumberLiteral => load_number_literal(&num.token.text, assist_reg, file),
        TTS::Pointer => load_pointer_name(vars, num, assist_reg, file)?,
        TTS::Address => load_address(vars, &num.branches[0].token.text, assist_reg, file)?,
        TTS::Name => load_unsigned(assist_reg, num, vars, functions, file)?,
        TTS::Parenthesis => (),
        TTS::RegisterResult => load_register_result(&num.token.text, assist_reg, file),
        _ => (),
    }
    Ok(())
}

fn gen_op_asm(
    operation: &Token,
    num1: &Branch,
    num2: &Branch,
    store_reg: &Register,
    assist_reg_1: &Register,
    assist_reg_2: &Register,
    vars: &Variables,
    functions: &Vec<Signature>,
    file: &mut File,
) -> Result<(), String> {
    file.write_all(format!(";{}\n", operation.text).as_bytes())
        .unwrap();
    load_branch(num1, assist_reg_1, vars, functions, file)?;
    load_branch(num2, assist_reg_2, vars, functions, file)?;

    match operation.text.as_str() {
        "+" => add(store_reg, assist_reg_1, assist_reg_2, file),
        "-" => sub(store_reg, assist_reg_1, assist_reg_2, file),
        "*" => mul(store_reg, assist_reg_1, assist_reg_2, file),
        "/" => div(store_reg, assist_reg_1, assist_reg_2, file),
        "%" => (),
        "==" => (),
        "<" => (),
        ">" => (),
        ">=" => (),
        "<=" => (),
        _ => return Err(format!("{} unrecognised operation", operation.text)),
    }
    Ok(())
}

fn real_op(
    operation: &Token,
    operation_stack: &mut Vec<Branch>,
    data_stack: &mut Vec<Branch>,
    vars: &mut Variables,
    signatures: &Vec<Signature>,
    store_branch: &Branch,
    store_reg: &Register,
    assist_reg_1: &Register,
    assist_reg_2: &Register,
    file: &mut File,
) -> Result<(), String> {
    loop {
        if operation_stack.len() == 0 {
            break;
        };
        if get_priority(&operation_stack.last().unwrap().token) >= get_priority(&operation) {
            let num2 = data_stack.pop().unwrap();
            let num1 = data_stack.pop().unwrap();
            let op = &operation_stack.pop().unwrap().token;
            gen_op_asm(
                op,
                &num1,
                &num2,
                &store_reg,
                &assist_reg_1,
                &assist_reg_2,
                vars,
                signatures,
                file,
            )?;
            data_stack.push(store_branch.clone());
        } else {
            break;
        }
    }
    Ok(())
}

pub fn operate<'a>(
    name: &str,
    args: &Vec<Branch>,
    min: usize,
    max: usize,
    vars: &mut Variables,
    signatures: &Vec<Signature>,
    store_reg: &Register,
    assist_reg_1: &Register,
    assist_reg_2: &Register,
    file: &mut File,
) -> Result<(), String> {
    let mut operation_stack: Vec<Branch> = Vec::new();
    let mut data_stack: Vec<Branch> = Vec::new();
    let store_branch = Branch::new(Token::register_result(&store_reg.name));
    let temp_branch = Branch::new(Token::name(name));

    for i in min..max {
        if is_operation(&args[i].token) {
            real_op(
                &args[i].token,
                &mut operation_stack,
                &mut data_stack,
                vars,
                signatures,
                &store_branch,
                &store_reg,
                &assist_reg_1,
                &assist_reg_2,
                file,
            )?;
            operation_stack.push(args[i].clone())
        } else if args[i].token.token_type == TTS::Parenthesis {
            if data_stack.len() > 0 {
                if data_stack[data_stack.len() - 1].token.token_type == TTS::RegisterResult {
                    store_reg_to_mem(vars, String::from(name), store_reg, file)?;
                    data_stack.pop();
                    data_stack.push(temp_branch.clone());
                }
            }
            operate(
                name,
                &args[i].branches[0].branches,
                0,
                args[i].branches[0].branches.len(),
                vars,
                signatures,
                &store_reg,
                &assist_reg_1,
                &assist_reg_2,
                file,
            )?;

            data_stack.push(store_branch.clone())
        } else if args[i].token.token_type == TTS::Pointer && args[i].branches[0].token.token_type == TTS::Parenthesis {
            if data_stack.len() > 0 {
                if data_stack[data_stack.len() - 1].token.token_type == TTS::RegisterResult {
                    store_reg_to_mem(vars, String::from(name), store_reg, file)?;
                    data_stack.pop();
                    data_stack.push(temp_branch.clone());
                }
            }
            load_pointer_op(name, vars, signatures, &args[i], store_reg, assist_reg_1, assist_reg_2, file)?;
        } else if args[i].token.token_type == TTS::Name && is_function(signatures, &args[i].token.text) {
            if data_stack.len() > 0 {
                if data_stack[data_stack.len() - 1].token.token_type == TTS::RegisterResult {
                    store_reg_to_mem(vars, String::from(name), store_reg, file)?;
                    data_stack.pop();
                    data_stack.push(temp_branch.clone());
                }
            }
            let reg = function_call(&args[i].token.text, signatures, vars, file, &args[i])?;
            data_stack.push(Branch::new(Token::register_result(&reg.name)));
        } else {
            data_stack.push(args[i].clone())
        }
    }

    if data_stack.len() != 1 {
        real_op(
            &Token::arithmetic_operation("end"),
            &mut operation_stack,
            &mut data_stack,
            vars,
            signatures,
            &store_branch,
            &store_reg,
            &assist_reg_1,
            &assist_reg_2,
            file,
        )?;
    } else {
        load_branch(
            &data_stack[0],
            &store_reg,
            vars,
            signatures,
            file,
        )?;
    }

    Ok(())
}

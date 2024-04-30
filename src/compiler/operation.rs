use super::{functions::Signature, register::Register, variables::Variables};
use crate::precompile::{
    branch::Branch,
    tokens::{Token, TTS},
};
use crate::types::{
    literals::{load_number_literal, load_register_result},
    unsigned::{
        add, sub, mul,
        load_unsigned
    },
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

fn load_branch(num: &Branch, store_reg: &Register, assist_reg: &Register, vars: &Variables, functions: &Vec<Signature>, file: &mut File) -> Result<(), String> {
    match num.token.token_type {
        TTS::NumberLiteral => load_number_literal(&num.token.text, assist_reg, file),
        TTS::Pointer => (),
        TTS::Address => (),
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
    println!(
        "op1: {:?} {:?} {:?}",
        num1.token.token_type, operation.token_type, num2.token.token_type
    );
    println!(
        "op2: {} {} {}",
        num1.token.text, operation.text, num2.token.text
    );

    load_branch(num1, store_reg, assist_reg_1, vars, functions, file)?;
    load_branch(num2, store_reg, assist_reg_2, vars, functions, file)?;

    match operation.text.as_str() {
        "+" => add(store_reg, assist_reg_1, assist_reg_2, file),
        "-" => sub(store_reg, assist_reg_1, assist_reg_2, file),
        "*" => mul(store_reg, assist_reg_1, assist_reg_2, file),
        "/" => (),
        "%" => (),
        "==" => (),
        "<" => (),
        ">" => (),
        ">=" => (),
        "<=" => (),
        _ => return Err(format!("{} unrecognised operation", operation.text)),
    }
    file.write_all(format!(";{}\n", operation.text).as_bytes())
        .unwrap();
    Ok(())
}

fn real_op<'a>(
    operation: &Token,
    operation_stack: &mut Vec<&Branch>,
    data_stack: &mut Vec<&'a Branch>,
    vars: &mut Variables,
    signatures: &Vec<Signature>,
    store_branch: &'a Branch,
    store_reg: &Register,
    assist_reg_1: &Register,
    assist_reg_2: &Register,
    file: &mut File,
) -> Result<(), String> {
    loop {
        if operation_stack.len() == 0 {
            break;
        };
        // println!("{} >= {} ?", &operation_stack.last().unwrap().token.text, &operation.text);
        // println!("{} >= {} ?", get_priority(&operation_stack.last().unwrap().token)?, get_priority(&operation)?);
        if get_priority(&operation_stack.last().unwrap().token) >= get_priority(&operation) {
            let num2 = data_stack.pop().unwrap();
            let num1 = data_stack.pop().unwrap();
            let op = &operation_stack.pop().unwrap().token;
            gen_op_asm(
                op, num1, num2, &store_reg, &assist_reg_1, &assist_reg_2, vars, signatures, file,
            )?;
            data_stack.push(store_branch);
        } else {
            break;
        }
    }
    Ok(())
}

pub fn operate(
    args: &Vec<Branch>,
    min: usize,
    max: usize,
    vars: &mut Variables,
    signatures: &Vec<Signature>,
    store_reg: Register,
    assist_reg_1: Register,
    assist_reg_2: Register,
    file: &mut File,
) -> Result<(), String> {
    let mut operation_stack: Vec<&Branch> = Vec::new();
    let mut data_stack: Vec<&Branch> = Vec::new();
    let store_branch = Branch::new(Token::register_result(&store_reg.name));

    for i in min..max {
        println!(
            "real: {:?} {}",
            args[i].token.token_type, args[i].token.text
        );
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
            operation_stack.push(&args[i])
        } else {
            data_stack.push(&args[i])
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
    }
    else {
        load_branch(data_stack[0], &assist_reg_1, &store_reg, vars, signatures, file)?;
    }

    Ok(())
}

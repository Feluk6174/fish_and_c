use crate::precompile::{branch::Branch, tokens::{Token, TTS}};
use super::{functions::Signature, variables::Variables};

fn is_operation(token: &Token) -> bool {
    token.token_type == TTS::ArithmeticOperation || token.token_type == TTS::Comparison
}

fn get_priority(token: &Token) -> Result<u8, String> {
    if token.token_type == TTS::ArithmeticOperation {
        if token.text == "+" {
            return Ok(1)
        }
        else if token.text == "*" {
            return Ok(2)
        }
        else if token.text == "end" {
            return  Ok(0)
        }
        else {
            return Err(format!("{} is not an arithmetic operation", token.text)) 
        }
    }
    else {
        return Ok(4)
    }
}

fn gen_op_asm(operation:&Token, num1:&Branch, num2:&Branch) {
    println!("op1: {:?} {:?} {:?}", num1.token.token_type, operation.token_type, num2.token.token_type);
    println!("op2: {} {} {}", num1.token.text, operation.text, num2.token.text);
    match num1.token.token_type {
        TTS::NumberLiteral => (),
        TTS::Pointer => (),
        TTS::Address => (),
        TTS::Name => (),
        TTS::Parenthesis => (),
        _ => (),
    }

}

fn real_op(operation:&Token, operation_stack:&mut Vec<&Branch>, data_stack:&mut Vec<&Branch>, vars: &mut Variables, signatures: &Vec<Signature>) -> Result<(), String> {
    loop {
        if operation_stack.len() == 0 {break};
        if get_priority(&operation_stack.last().unwrap().token) >= get_priority(&operation) {
            let num2 = data_stack.pop().unwrap();
            let num1 = data_stack.pop().unwrap();
            let op = &operation_stack.pop().unwrap().token;
            gen_op_asm(op, num1, num2);
            data_stack.push(num1);
        }
        else {
            break
        }
    }
    Ok(())
}

pub fn operate(args: &Vec<Branch>, min: usize, max: usize, vars: &mut Variables, signatures: &Vec<Signature>) -> Result<(), String> {
    let mut operation_stack:Vec<&Branch> = Vec::new();
    let mut data_stack:Vec<&Branch> = Vec::new();
    
    for i in min..max {
        println!("real: {:?} {}", args[i].token.token_type, args[i].token.text);
        if is_operation(&args[i].token) {
            real_op(&args[i].token, &mut operation_stack, &mut data_stack, vars, signatures)?;
            operation_stack.push(&args[i])
        }
        else {
            data_stack.push(&args[i])
        }
    }

    real_op(&Token::arithmetic_operation("end"), &mut operation_stack, &mut data_stack, vars, signatures)?;

    Ok(())
}
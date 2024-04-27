use crate::precompile::{branch::Branch, tokens::{Token, TTS}};

fn is_operation(token: &Token) -> bool {
    token.token_type == TTS::ArithmeticOperation || token.token_type == TTS::Comparison
}

fn get_priority(token: &Token) -> Result<u8, String> {
    if token.token_type == TTS::ArithmeticOperation {
        if token.text == "+" {
            return Ok(0)
        }
        else if token.text == "*" {
            return Ok(1)
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
    match num1.token.token_type {
        TTS::NumberLiteral => (),
        TTS::Pointer => (),
        TTS::Address => (),
        TTS::Name => (),
        TTS::Parenthesis => (),
        _ => (),
    }

}

fn real_op(operation:Token, mut operation_stack:Vec<&Branch>, mut data_stack:Vec<&Branch>) -> Result<(), String> {
    loop {
        if operation_stack.len() == 0 {break};
        if get_priority(&operation_stack.last().unwrap().token) >= get_priority(&operation) {
            let num2 = data_stack.pop().unwrap();
            let num1 = data_stack.pop().unwrap();
            let op = &operation_stack.pop().unwrap().token;
        }
    }
    Ok(())
}

pub fn operate(args: Vec<Branch>, min: usize, max: usize) -> Result<(), String> {
    let mut operation_stack:Vec<&Branch> = Vec::new();
    let mut data_stack:Vec<&Branch> = Vec::new();
    
    for i in min..max {
        if is_operation(&args[i].token) {
            operation_stack.push(&args[i])
        }
        else {
            data_stack.push(&args[i])
        }
    }
    Ok(())
}
use crate::precompile::tokens::{Token, TTS};
use crate::precompile::branch::Branch;

use super::branch;

fn get_type_branch(
    tokens: &Vec<Token>,
    idx: usize,
    mut depth: usize,
) -> Result<(Branch, usize), String> {
    let mut branches: Branch;
    if tokens[idx].token_type == TTS::VarType {
        branches = Branch::new(tokens[idx].clone());
    } else if tokens[idx].token_type == TTS::Pointer {
        if tokens[idx + 1].token_type == TTS::VarType {
            branches = Branch::new(tokens[idx].clone());
            match get_type_branch(tokens, idx + 1, depth + 1) {
                Ok(branch) => {
                    branches.branches.push(branch.0);
                    depth += branch.1;
                }
                Err(err) => return Err(err),
            }
        } else {
            return Err(format!("*{} Is not a valid type", tokens[idx + 1].text));
        }
    } else {
        return Err(format!("{} Is not a valid type", tokens[idx + 1].text));
    }
    Ok((branches, depth))
}

fn func_branch(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut branches: Vec<Branch> = Vec::new();
    let token: Token = tokens[idx - 1].clone();
    let mut depth: usize = 0;

    match get_type_branch(tokens, idx + depth, 0) {
        Ok(branch) => {
            branches.push(branch.0);
            depth += branch.1 + 1;
        }
        Err(_) => return Err(String::from("Function no returntype specipfied")),
    };

    if tokens[idx + depth].token_type == TTS::Name {
        branches.push(Branch {
            token: tokens[idx + depth].clone(),
            branches: Vec::new(),
        });
        depth += 1;
    } else {
        return Err(String::from("Function needs name"));
    }

    if !(tokens[idx + depth].token_type == TTS::Parenthesis && tokens[idx + depth].text == "(") {
        return Err(String::from("Expected parenthesis in function declaration"));
    }
    depth += 1;

    match declare_arguments(&tokens, idx + depth) {
        Ok(branch) => {
            branches.push(branch.0);
            depth += branch.1;
        }
        Err(err) => return Err(err),
    }
    Ok((
        Branch {
            token: token,
            branches: branches,
        },
        depth,
    ))
}

fn declare_arguments(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut parent: Branch = Branch::new(Token::declaration_args());
    let mut depth: usize = 0;
    let mut last: Branch;
    while !(tokens[idx + depth].token_type == TTS::Parenthesis && tokens[idx + depth].text == ")") {
        match get_type_branch(tokens, idx, 0) {
            Ok(branch) => {
                last = branch.0;
                depth += branch.1 + 1;
            }
            Err(err) => return Err(err),
        }
        if tokens[idx + depth].token_type == TTS::Name {
            last.branches.push(Branch::new(tokens[idx + depth].clone()));
            depth += 1;
        } else {
            return Err(String::from("Function arguments need name"));
        }
        parent.branches.push(last)
    }
    Ok((parent, depth + 1))
}

fn operation(
    tokens: &Vec<Token>,
    idx: usize,
    mut parent: Branch,
) -> Result<(Branch, usize), String> {
    let mut depth: usize = 0;
    if tokens[idx].token_type == TTS::NewCommand {
        return Err(String::from("Empty declaration"));
    }
    while !(tokens[idx + depth].token_type == TTS::NewCommand) {
        let mut temp = Branch::new(tokens[idx + depth].clone());
        if tokens[idx + depth].token_type == TTS::Pointer && tokens[idx + depth + 1].token_type == TTS::Parenthesis && tokens[idx + depth + 1].text == "("{
            depth += 1;
            match parenthesis_operation(tokens, idx+depth, Branch::new(tokens[idx+depth].clone()))? {
                branch => {
                    temp.branches.push(branch.0);
                    depth += branch.1;
                }
            }
            parent.branches.push(temp);
        }
        else if tokens[idx + depth].token_type == TTS::Pointer {
            depth += 1;
            temp.branches.push(Branch::new(tokens[idx + depth].clone()));
            parent.branches.push(temp);
        }
        else if tokens[idx + depth].token_type == TTS::Address {
            depth += 1;
            temp.branches.push(Branch::new(tokens[idx + depth].clone()));
            parent.branches.push(temp);
        }
        else if tokens[idx + depth].token_type == TTS::Name && tokens[idx + depth + 1].token_type == TTS::Parenthesis && tokens[idx + depth + 1].text == "(" {
            let branch = func_call_tree(tokens, idx+depth)?;
            parent.branches.push(branch.0);
            depth += branch.1;
        }
        else {
            parent.branches.push(temp);
        }
        depth += 1;
    }
    Ok((parent, depth))
}

fn parenthesis_operation(
    tokens: &Vec<Token>,
    idx: usize,
    mut parent: Branch,
) -> Result<(Branch, usize), String> {
    let mut depth: usize = 0;
    if tokens[idx].token_type == TTS::Parenthesis && tokens[idx].text == ")" {
        return Err(String::from("Empty paretethis"));
    }
    depth += 1;
    while !(tokens[idx + depth].token_type == TTS::Parenthesis && tokens[idx+depth].text == ")") {
        let mut temp = Branch::new(tokens[idx + depth].clone());
        if tokens[idx + depth].token_type == TTS::Pointer && tokens[idx + depth + 1].token_type == TTS::Parenthesis && tokens[idx + depth + 1].text == "("{
            depth += 1;
            match parenthesis_operation(tokens, idx+depth, Branch::new(tokens[idx+depth].clone()))? {
                branch => {
                    temp.branches.push(branch.0);
                    depth += branch.1;
                }
            }
            parent.branches.push(temp);
        }
        else if tokens[idx + depth].token_type == TTS::Pointer {
            depth += 1;
            temp.branches.push(Branch::new(tokens[idx + depth].clone()));
            parent.branches.push(temp);
        }
        else if tokens[idx + depth].token_type == TTS::Address {
            depth += 1;
            temp.branches.push(Branch::new(tokens[idx + depth].clone()));
            parent.branches.push(temp);
        }
        else if tokens[idx + depth].token_type == TTS::Name && tokens[idx + depth + 1].token_type == TTS::Parenthesis && tokens[idx + depth + 1].text == "(" {
            let branch = func_call_tree(tokens, idx+depth)?;
            parent.branches.push(branch.0);
            depth += branch.1;
        }
        else {
            parent.branches.push(temp);
        }
        depth += 1;
    }
    Ok((parent, depth))
}

fn declare_variable(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut parent: Branch = Branch::new(tokens[idx].clone());
    let mut depth: usize = 0;
    match get_type_branch(tokens, idx, 0) {
        Ok(branch) => {
            parent.branches.push(branch.0);
            depth += branch.1 + 1;
        }
        Err(err) => return Err(err),
    }

    if tokens[idx + depth].token_type != TTS::Name {
        return Err(String::from("Variable needs name"));
    }

    parent
        .branches
        .push(Branch::new(tokens[idx + depth].clone()));
    depth += 1;

    if tokens[idx + depth].token_type != TTS::Assignation {
        return Err(String::from("Variable {}"));
    }
    depth += 1;

    match operation(tokens, idx + depth, parent) {
        Ok(branch) => {
            parent = branch.0;
            depth += branch.1;
        }
        Err(err) => return Err(err),
    }

    Ok((parent, depth))
}

fn return_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut parent: Branch = Branch::new(tokens[idx].clone());
    let mut depth: usize = 1;

    match operation(tokens, idx + depth, parent) {
        Ok(branch) => {
            parent = branch.0;
            depth += branch.1;
        }
        Err(err) => return Err(err),
    };

    return Ok((parent, depth));
}

fn if_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut depth: usize = 0;
    let mut parent = Branch::new(tokens[idx].clone());
    depth += 1;

    match operation(tokens, idx + depth, parent) {
        Ok(branch) => {
            parent = branch.0;
            depth += branch.1 + 1;
        }
        Err(err) => return Err(err),
    };

    if !(tokens[idx + depth].token_type == TTS::Keys && tokens[idx + depth].text == "{") {
        return Err(String::from("Expected { after if statement."));
    }

    match code_block(tokens, idx + depth) {
        Ok(branch) => {
            parent.branches.push(branch.0);
            depth += branch.1;
        }
        Err(err) => return Err(err),
    };

    if tokens[idx + depth + 1].token_type == TTS::ElseKeyword {
        depth += 1;
        match else_tree(tokens, idx + depth) {
            Ok(branch) => {
                parent.branches.push(branch.0);
                depth += branch.1;
            }
            Err(err) => return Err(err),
        }
    }

    Ok((parent, depth))
}

fn else_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut parent = Branch::new(tokens[idx].clone());
    let mut depth: usize = 1;

    if tokens[idx + depth].token_type == TTS::IfKeyword {
        match if_tree(tokens, idx + depth) {
            Ok(branch) => {
                parent.branches.push(branch.0);
                depth += branch.1;
            }
            Err(err) => return Err(err),
        };
    } else if tokens[idx + depth].token_type == TTS::Keys && tokens[idx + depth].text == "{" {
        match code_block(tokens, idx + depth) {
            Ok(branch) => {
                parent.branches.push(branch.0);
                depth += branch.1;
            }
            Err(err) => return Err(err),
        }
    } else {
        return Err(String::from("Expected { or if after an else"));
    }

    Ok((parent, depth))
}

fn while_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut depth: usize = 0;
    let mut parent = Branch::new(tokens[idx].clone());
    depth += 1;

    match operation(tokens, idx + depth, parent) {
        Ok(branch) => {
            parent = branch.0;
            depth += branch.1 + 1;
        }
        Err(err) => return Err(err),
    };

    if !(tokens[idx + depth].token_type == TTS::Keys && tokens[idx + depth].text == "{") {
        return Err(String::from("Expected { after if statement."));
    }

    match code_block(tokens, idx + depth) {
        Ok(branch) => {
            parent.branches.push(branch.0);
            depth += branch.1;
        }
        Err(err) => return Err(err),
    };

    Ok((parent, depth))
}

fn signle_token(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    if tokens[idx + 1].token_type != TTS::NewCommand {
        return Err(format!("expected ; after {}", tokens[idx].text));
    }
    Ok((Branch::new(tokens[idx].clone()), 1))
}

fn name_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    if tokens[idx + 1].token_type == TTS::Assignation {
        assignation_tree(tokens, idx)
    }
    else if tokens[idx + 1].token_type == TTS::Parenthesis {
        func_call_tree(tokens, idx)
    }
    else {
        Err(String::from("Expected = or ( after name"))
    }
}

fn assignation_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut parent = Branch::new(tokens[idx].clone());
    let mut depth:usize = 2;
    match operation(tokens, idx+depth, parent) {
        Ok(branch) => {
            parent = branch.0;
            depth += branch.1-1;
        }
        Err(err) => return Err(err)
    }
    Ok((parent, depth))
}

fn func_call_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut parent = Branch::new(tokens[idx].clone());
    let mut depth:usize = 2;

    while !(tokens[idx+depth].token_type == TTS::Parenthesis && tokens[idx+depth].text == ")"){
        let mut parenth = Branch::new(tokens[idx+1].clone());
        match operation(tokens, idx+depth, parenth) {
            Ok(branch) => {
                parenth = branch.0;
                depth += branch.1+1;
            }
            Err(err) => return Err(err)
        }
        parent.branches.push(parenth);
    }
    
    Ok((parent, depth))
}

fn assembly_tree(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut parent = Branch::new(tokens[idx].clone());
    if tokens[idx+1].token_type != TTS::Keys || tokens[idx+1].text != "{" {
        return Err(String::from("Expected { after asm"));
    }
    if tokens[idx+3].token_type != TTS::Keys || tokens[idx+3].text != "}" {
        return Err(String::from("Expected } after asm"));
    }
    if tokens[idx+2].token_type != TTS::AssemblyCode {
        return Err(String::from("Expected assembly code after asm"));
    }
    parent.branches.push(Branch::new(tokens[idx+2].clone()));
    Ok((parent, 4))

}

fn code_block(tokens: &Vec<Token>, idx: usize) -> Result<(Branch, usize), String> {
    let mut branch: Branch = Branch::new(tokens[idx].clone());
    let mut depth: usize = 0;
    loop {
        match tokens[idx + depth].token_type {
            TTS::VarType | TTS::Pointer => match declare_variable(tokens, depth + idx) {
                Ok(operations) => {
                    branch.branches.push(operations.0);
                    depth += operations.1;
                }
                Err(err) => return Err(err),
            },
            TTS::Keys => {
                if tokens[idx + depth].text == "{" {
                } else if tokens[idx + depth].text == "}" {
                    break;
                }
            }
            TTS::Name => match name_tree(tokens, idx + depth) {
                Ok(operations) => {
                    branch.branches.push(operations.0);
                    depth += operations.1 + 1;
                }
                Err(err) => return Err(err),
            },
            TTS::IfKeyword => match if_tree(tokens, idx + depth) {
                Ok(operations) => {
                    branch.branches.push(operations.0);
                    depth += operations.1;
                }
                Err(err) => return Err(err),
            },
            TTS::WhileKeyword => match while_tree(tokens, idx + depth) {
                Ok(operations) => {
                    branch.branches.push(operations.0);
                    depth += operations.1;
                }
                Err(err) => return Err(err),
            },
            TTS::ContinueKeyword | TTS::BreakKeyword => match signle_token(tokens, idx + depth) {
                Ok(operations) => {
                    branch.branches.push(operations.0);
                    depth += operations.1;
                }
                Err(err) => return Err(err),
            },
            TTS::ReturnKeyword => match return_tree(tokens, idx + depth) {
                Ok(operations) => {
                    branch.branches.push(operations.0);
                    depth += operations.1;
                }
                Err(err) => return Err(err),
            },
            TTS::Assembly => match assembly_tree(tokens, idx+depth) {
                Ok(operations) => {
                    branch.branches.push(operations.0);
                    depth += operations.1;
                }
                Err(err) => return Err(err),
            }
            _ => return Err(format!("{:?} Not implmented", tokens[idx+depth].text)),
        }
        depth += 1;
    }
    Ok((branch, depth))
}

pub fn build_tree(tokens: Vec<Token>) -> Result<Vec<Branch>, (String, Vec<Branch>)> {
    let mut idx: usize = 0;
    let mut branches: Vec<Branch> = Vec::new();
    while idx < tokens.len() {
        if tokens[idx].token_type == TTS::Function {
            match func_branch(&tokens, idx + 1) {
                Ok(branch) => {
                    branches.push(branch.0);
                    idx += branch.1
                }
                Err(err) => return Err((err, branches)),
            }
        }
        if tokens[idx].token_type == TTS::Keys && tokens[idx].text == "{" {
            match code_block(&tokens, idx) {
                Ok(branch) => {
                    branches.push(branch.0);
                    idx += branch.1
                }
                Err(err) => return Err((err, branches)),
            }
        } else {
            idx += 1;
        }
    }
    Ok(branches)
}

pub fn print_tree(tree: &Vec<Branch>, depth: usize) {
    for branch in tree {
        println!("{}{:?}", " ".repeat(depth * 4), branch.token);

        print_tree(&branch.branches, depth + 1)
    }
}

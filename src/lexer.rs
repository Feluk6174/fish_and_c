use crate::tokens::Token;
use std::fs;
use std::str;

pub fn read_file<S: AsRef<str>>(file_name: S) -> String {
    fs::read_to_string(file_name.as_ref()).expect("Should have been able to read the file")
}

fn add_token(tokens: &mut Vec<Token>, mut token: String, definitions:&Vec<(String, String)>) {
    for (name, value) in definitions {
        if name == &token {
            token = value.to_owned();
        }
    }
    if token == "" {
        return;
    }
    if !token
        .as_bytes()
        .iter()
        .any(|&x| !((x >= b'0' && x <= b'9') || x == b'.'))
        || token.as_bytes()[0] == b'\"' && token.as_bytes()[token.as_bytes().len() - 1] == b'\"'
        || token.as_bytes()[0] == b'\'' && token.as_bytes()[token.as_bytes().len() - 1] == b'\''
    {
        tokens.push(Token::literal(&token));
        return;
    }

    tokens.push(match token.as_str() {
        "u8" | "u16" | "u32" | "u64" | "f8" | "f16" | "f32" | "f64" => Token::var_type(&token),
        "fn" => Token::function(&token),
        "if" => Token::if_keyword(&token),
        "else" => Token::else_keyword(&token),
        "while" => Token::while_keyword(&token),
        "for" => Token::for_keyword(&token),
        "return" => Token::return_keyword(&token),
        "break" => Token::break_keyword(&token),
        "continue" => Token::continue_keyword(&token),
        _ => Token::name(&token),
    })
}

pub fn tokenizer(code: String, definitions:Vec<(String, String)>) -> Vec<Token> {
    let mut last_ch = ' ';
    let mut text: Vec<u8> = Vec::new();
    let mut tokens = Vec::new();
    for ch_u8 in code.as_bytes() {
        let ch: char = *ch_u8 as char;
        if last_ch != ' ' && ch == '=' {
            tokens.push(Token::comparison(&format!("{}{}", last_ch, ch)));
            last_ch = ' ';
            continue;
        } else if last_ch == '=' {
            tokens.push(Token::assignation("="));
            last_ch = ' ';
        } else if last_ch != ' ' {
            tokens.push(Token::comparison(&format!("{}", last_ch)));
            last_ch = ' ';
        }

        match ch {
            ' ' | '\n' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new()
            }
            '(' | ')' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new();
                tokens.push(Token::parenthesis(&ch.to_string()));
            }
            ';' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new();
                tokens.push(Token::new_command(&ch.to_string()));
            }
            '&' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new();
                tokens.push(Token::address(&ch.to_string()));
            }
            '$' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new();
                tokens.push(Token::pointer(&ch.to_string()));
            }
            '[' | ']' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new();
                tokens.push(Token::square_keys(&ch.to_string()));
            }
            '{' | '}' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new();
                tokens.push(Token::keys(&ch.to_string()));
            }
            '+' | '-' | '*' | '/' | '%' => {
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new();
                tokens.push(Token::arithmetic_operation(&ch.to_string()));
            }
            '<' | '>' | '=' | '!' => {
                last_ch = ch;
                add_token(&mut tokens, String::from_utf8(text).unwrap(), &definitions);
                text = Vec::new()
            }
            _ => text.push(*ch_u8),
        }
    }
    tokens
}

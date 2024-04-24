use crate::precompile::tokens::{Token, TTS};

#[derive(Debug, Clone)]
pub struct Branch {
    pub token: Token,
    pub branches: Vec<Branch>,
}

impl Branch {
    pub fn new(token: Token) -> Self {
        Self {
            token: token,
            branches: Vec::new(),
        }
    }
}

pub fn get_name_from_arg(mut tree:&Branch) -> Result<String, String> {
    while tree.branches.len() != 0 {
        tree = &tree.branches[0];
    }
    if tree.token.token_type != TTS::Name {
        return Err(String::from("Expected name after type"));
    }
    Ok(tree.token.text.clone())
}
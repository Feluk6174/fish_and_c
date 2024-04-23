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
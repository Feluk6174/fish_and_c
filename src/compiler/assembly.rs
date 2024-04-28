use std::{fs::File, io::Write};
use crate::precompile::branch::Branch;

pub fn gen_asm_asm(tree: &Branch, file:&mut File) -> Result<(), String> {
    file.write_all(tree.branches[0].token.text.as_bytes()).expect("Error writing to file");
    Ok(())
}
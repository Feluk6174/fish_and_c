use std::fs::File;
use crate::precompile::branch::Branch;
use super::functions::{Function, build_functions};

pub fn compile(tree: Vec<Branch>, file_name: &str) -> Result<(), String> {
    let mut functions:Vec<Function> = build_functions(&tree)?;
    let mut file = File::create(file_name).expect("Error writing to file");
    Ok(())
}

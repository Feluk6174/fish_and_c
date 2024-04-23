use crate::precompile::branch::Branch;
use crate::compiler::util::Function;


pub fn compile(tree:Vec<Branch>) -> Result<(), String> {
    let mut functions = Vec::new();
    for i in 0..tree.len()/2 {
        println!("{}", i);
        functions.push(Function::new(&tree, i*2)?);
    }
    println!("{:?}", functions);
    Ok(())
}
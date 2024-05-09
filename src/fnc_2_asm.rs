use crate::compiler::compile::compile;
use crate::precompile::{lexer, syntax_tree};
use crate::precompile::preprocessor;
pub fn main(code_path: &str, asm_path: &str) {
    let res = lexer::read_file(code_path);
    let (code, definitions) = preprocessor::preprocess(res.clone()).unwrap();
    let tks = match lexer::tokenizer(code, definitions) {
        Ok(tks) => tks,
        Err(err) => panic!("{}", err),
    };
    println!("{:?}", tks);
    let tree = match syntax_tree::build_tree(tks) {
        Ok(coses) => coses,
        Err(err) => {
            println!("{}", err.0);
            err.1
        }
    };
    syntax_tree::print_tree(&tree, 0);
    match compile(tree, asm_path) {
        Ok(_) => println!("Compiled"),
        Err(err) => panic!("{}", err)
    };
}
mod precompile;
mod compiler;
use precompile::{lexer, syntax_tree, preprossesor};
use compiler::compile;
fn main() {
    let res = lexer::read_file("test.fnc");
    let (code, definitions) = preprossesor::preprosess(res.clone()).unwrap();
    let tks = lexer::tokenizer(code, definitions).unwrap();
    println!("{:?}", tks);
    let tree = match syntax_tree::build_tree(tks) {
        Ok(coses) => coses,
        Err(err) => {
            println!("{}", err.0);
            err.1
        }
    };
    syntax_tree::print_tree(&tree, 0);

    //compile::compile(tree, "out.asm").unwrap();
}

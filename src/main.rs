mod preprossesor;
mod lexer;
mod tokens;
mod syntax_tree;
fn main() {
    let res = lexer::read_file("test.yeso");
    let (code, definitions) = preprossesor::preprosess(res.clone()).unwrap();
    let tks = lexer::tokenizer(code, definitions);
    println!("{:?}", tks);
    let tree = match syntax_tree::build_tree(tks) {
        Ok(coses) => coses,
        Err(err) => {
            println!("{}", err.0);
            err.1
        }
    };
    syntax_tree::print_tree(&tree, 0)
}

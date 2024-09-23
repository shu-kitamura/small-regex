mod parser;
mod compiler;

use parser::parse;
use compiler::compile;

fn main() {
    let ast = parse("a|b");
    println!("{:?}", ast);

    let inst = compile(&ast);
    println!("{:?}", inst);
}

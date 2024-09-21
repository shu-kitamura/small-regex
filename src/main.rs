mod parser;

use parser::parse;

fn main() {
    let ast = parse("a|b");
    println!("{:?}", ast)
}

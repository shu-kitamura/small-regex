mod parser;

use parser::parse;

fn main() {
    let ast = parse("abc(def|ghi)");
    println!("{:?}", ast)
}

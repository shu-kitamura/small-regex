mod parser;
mod compiler;
mod evaluator;

use parser::parse;
use compiler::compile;
use evaluator::eval;

fn main() {
    println!("{}", pattern_match("ab*(de|fg)", "abbbfg")); // true
    println!("{}", pattern_match("a?b(d*e|fg)", "bdde"));  // true
    println!("{}", pattern_match("a?b(d*e|fg)", "cbfg"));  // false
}

fn pattern_match(pattern: &str, line: &str) -> bool {
    let ast = parse(pattern);
    let instructions = compile(&ast);
    let chars: Vec<char> = line.chars().collect();
    eval(&instructions, &chars, 0, 0)
}

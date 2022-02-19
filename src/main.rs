use std::env;
use std::fs::read_to_string;
mod parse;

fn main() {

    let file = env::args().nth(1).unwrap();
    let content = read_to_string(file).unwrap();
    let mut _lex = parse::Interpret::new(content);
     _lex.inter();
}

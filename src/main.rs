use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;
mod utils;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let lexer = Lexer::new(input.as_str());
    let mut parse = Parser::new(lexer);
    println!("{:#?}", parse.parse_stmt());
}

use lexer::Lexer;

mod lexer;
mod utils;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let lexer = Lexer::new(input.as_str());
    for a in lexer {
        println!("\"{}\"", a.span.as_str());
        println!("{:?}", a);
    }
}

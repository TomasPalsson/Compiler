use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::parse_program;
mod tokens; 
mod lexer;
mod parser;
mod ast;

fn read_input() -> String {
    print!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.to_string()

}
fn main() {
    let mut input = read_input().trim().to_string();

    while !input.ends_with('\n') {
        let next_line = read_input().trim().to_string();
        if next_line == "stop" {
            break;
        }
        input += &next_line;
        println!("input: {:?}", input);
    }

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenise();
    println!("Tokens: {:?}", tokens);
    let ast = parse_program(&tokens);
    println!("Abstract Syntax tree: {:?}", ast);
}


#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Identifier(String),
    Function,
    EndFunction,
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    LParen,
    RParen,
    Comma,
    Semicolon,
    End,
}


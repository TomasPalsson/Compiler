use crate::tokens::Token;
use std::{self, iter::Peekable};
use std::slice::Iter;
use crate::ast::{Statement, Expression};

pub fn parse_program(tokens: &[Token]) -> Option<Vec<Statement>>{
    let mut iter = tokens.iter().peekable();
    let mut statements = Vec::new();

    while iter.peek().is_some() {
        let stmt = parse_statement(&mut iter);
        statements.push(stmt.expect("Unexpected Statement"));
    }

    Some(statements)
}


fn parse_statement(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    
    let statement = match iter.peek() {
        Some(Token::Function) => parse_function(iter),
        Some(Token::Identifier(_)) => parse_assignment(iter),
        _ => Err(format!("Cannot parse found {:?}", iter.peek())),
    }?;

    if let Some(Token::Function) = iter.peek() {
        return Ok(statement);
    }

    match iter.peek() {
        Some(Token::EndFunction) => {
            iter.next();
            Ok(statement)
        }
        Some(Token::Semicolon) => {
            iter.next();
            Ok(statement)
        }
        Some(tok) => Err(format!("Expected semicolon, got {:?}", tok)),
        None => Ok(statement),
    }
}

fn parse_function(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    iter.next();
    let name = match iter.peek() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => "Name".to_string(),
    };
    iter.next();
    let mut args = Vec::new();
    if matches!(iter.peek(), Some(Token::LParen)) {
        iter.next();
        while !matches!(**iter.peek().unwrap(), Token::RParen) {
            let arg = parse_arg(iter);
            if let Ok(arg) = &arg {
                args.push(arg.clone());
            } else if let Err(err) = &arg {
                panic!("{}", err);
            }

            if matches!(**iter.peek().unwrap(), Token::Comma) {
                iter.next();
            } else if !matches!(iter.peek().unwrap(), Token::RParen) {
                return Err("Function Setup incorrectly".to_string())
            }
        }
        iter.next();
    }


    let mut body = Vec::new();
    while !matches!(**iter.peek().unwrap(), Token::EndFunction) {
        let statement = parse_statement(iter);
        body.push(statement?);
    }
    Ok(Statement::Function{
        name,
        params: args,
        body
    })
}

fn parse_arg(iter: &mut Peekable<Iter<Token>>) -> Result<Expression, String> {
    match iter.peek().unwrap() {
        Token::Identifier(na) => {
            let name = na.clone();
            iter.next();
            Ok(Expression::FunctionArg(name))
        },
        _ => Err("Arg defined incorrectly".to_string()),
    }

}

fn parse_assignment(iter: &mut Peekable<Iter<Token>>) -> Result<Statement, String> {
    match iter.peek().unwrap() {
        Token::Identifier(name) => {
            iter.next();
            if matches!(iter.peek().unwrap(), Token::Assign) {
                iter.next();
                match iter.peek() {
                    Some(Token::Number(int)) => {
                        iter.next();
                        Ok(Statement::Assign { name: name.to_string(), value: Expression::Integer(*int)})
                    }
                    Some(Token::Identifier(var)) => {
                        iter.next();
                        Ok(Statement::Assign {name: name.to_string(), value: Expression::Variable(var.to_string())})
                    }
                    _ => Err(format!("Assignment parsing failed got: {:?}", iter.peek()))
                } 
            } else {

                Err(format!("No = Operator found: {:?}", iter.peek().unwrap()))
            }
        }
        _ => Err("Error in parsing assignment!".to_string())
    }
}


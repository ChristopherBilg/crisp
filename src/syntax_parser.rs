use crate::atom::Atom;
use crate::lexical_analyzer::{tokenize, Token};

use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    pub err: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.err)
    }
}

impl Error for ParseError {}

pub fn parse(program: &str) -> Result<Atom, ParseError> {
    let token_result = tokenize(program);
    let mut tokens;

    match token_result {
        Ok(_) => tokens = token_result.unwrap().into_iter().rev().collect::<Vec<_>>(),
        Err(_) => {
            return Err(ParseError {
                err: format!("{}", token_result.err().unwrap()),
            })
        }
    }

    let parsed_list = parse_list(&mut tokens)?;
    Ok(parsed_list)
}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Atom, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Expected '(', found {:?}", token),
        });
    }

    let mut list: Vec<Atom> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(ParseError {
                err: format!("Did not find enough tokens."),
            });
        }

        let t = token.unwrap();
        match t {
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RParen => {
                return Ok(Atom::List(list));
            }
            Token::Integer(n) => list.push(Atom::Integer(n)),
            Token::Float(n) => list.push(Atom::Float(n)),
            Token::Symbol(s) => list.push(Atom::Symbol(s)),
        }
    }

    Ok(Atom::List(list))
}

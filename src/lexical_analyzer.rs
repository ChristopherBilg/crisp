use std::{fmt, num::ParseIntError};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(i128),
    Float(f64),
    Symbol(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::Symbol(s) => write!(f, "{}", s),
        }
    }
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, ParseIntError> {
    let program = program.replace("(", " ( ").replace(")", " ) ");
    let words = program.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let integer = word.parse::<i128>();
                if integer.is_ok() {
                    tokens.push(Token::Integer(integer.unwrap()));
                    continue;
                }

                let float = word.parse::<f64>();
                if float.is_ok() {
                    tokens.push(Token::Float(float.unwrap()));
                    continue;
                }

                tokens.push(Token::Symbol(word.to_string()));
            }
        }
    }

    Ok(tokens)
}

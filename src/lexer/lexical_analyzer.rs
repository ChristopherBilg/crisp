use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(i) => write!(f, "{}", i),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
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
                let i = word.parse::<i64>();
                if i.is_ok() {
                    tokens.push(Token::Integer(i.unwrap()));
                } else {
                    tokens.push(Token::Symbol(word.to_string()));
                }
            }
        }
    }
    
    Ok(tokens)
}

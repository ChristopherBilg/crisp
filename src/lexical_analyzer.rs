use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(i64),
    Float(f64),
    Symbol(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            _ => write!(f, "{}", self),
        }
    }
}

pub fn tokenize(program: &str) -> Vec<Token> {
    let program = program.replace("(", " ( ").replace(")", " ) ");
    let words = program.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let integer = word.parse::<i64>();
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

    tokens
}

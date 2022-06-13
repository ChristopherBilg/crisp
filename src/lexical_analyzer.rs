use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Integer(n) => write!(f, "{}", n),
            Token::Float(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "{}", s),
            Token::Symbol(s) => write!(f, "{}", s),
        }
    }
}

pub fn tokenize(program: &str) -> Vec<Token> {
    let program = program.replace('(', " ( ").replace(')', " ) ");
    let words = program.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();

    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let integer = word.parse::<i64>();
                if let Ok(integer) = integer {
                    tokens.push(Token::Integer(integer));
                    continue;
                }

                let float = word.parse::<f64>();
                if let Ok(float) = float {
                    tokens.push(Token::Float(float));
                    continue;
                }

                if word.contains("example") {
                    tokens.push(Token::String(word.to_string()));
                }

                tokens.push(Token::Symbol(word.to_string()));
            }
        }
    }

    tokens
}

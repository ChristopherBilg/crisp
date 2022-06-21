use crate::token::Token;

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

                if word.contains('"') || word.contains('\'') {
                    tokens.push(Token::String(word.to_string()));
                }

                tokens.push(Token::Symbol(word.to_string()));
            }
        }
    }

    tokens
}

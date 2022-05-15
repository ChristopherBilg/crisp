use crate::atom::Atom;
use crate::lexical_analyzer::{tokenize, Token};

pub fn parse(program: &str) -> Atom {
    let token_result = tokenize(program);
    let mut tokens = token_result.into_iter().rev().collect::<Vec<_>>();

    let parsed_list = parse_list(&mut tokens);
    parsed_list
}

fn parse_list(tokens: &mut Vec<Token>) -> Atom {
    // Pop the initial LParen
    let _ = tokens.pop();

    let mut list: Vec<Atom> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();
        match token {
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens);
                list.push(sub_list);
            }
            Token::RParen => {
                return Atom::List(list);
            }
            Token::Integer(n) => list.push(Atom::Integer(n)),
            Token::Float(n) => list.push(Atom::Float(n)),
            Token::Symbol(s) => list.push(Atom::Symbol(s)),
        }
    }

    Atom::List(list)
}

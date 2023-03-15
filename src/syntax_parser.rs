use crate::atom::Atom;
use crate::lexical_analyzer::{tokenize, Token};

pub fn parse(program: &str) -> Atom {
    let token_result = tokenize(program);
    let mut tokens = token_result.into_iter().rev().collect::<Vec<_>>();

    parse_list(&mut tokens)
}

fn parse_list(tokens: &mut Vec<Token>) -> Atom {
    // Pop the initial LParen
    tokens.pop();

    let mut list: Vec<Atom> = Vec::new();
    while !tokens.is_empty() {
        match tokens.pop() {
            Some(Token::LParen) => {
                // Add back an LParen since it's part of a sublist
                tokens.push(Token::LParen);

                let sub_list = parse_list(tokens);
                list.push(sub_list);
            }
            Some(Token::RParen) => return Atom::List(list),
            Some(Token::Integer(n)) => list.push(Atom::Integer(n)),
            Some(Token::Float(n)) => list.push(Atom::Float(n)),
            Some(Token::String(s)) => list.push(Atom::String(s)),
            Some(Token::Symbol(s)) => list.push(Atom::Symbol(s)),
            None => continue,
        }
    }

    Atom::List(list)
}

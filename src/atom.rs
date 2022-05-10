use std::fmt;

use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Void,
    Integer(BigInt),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Atom>),
    List(Vec<Atom>),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Void => write!(f, "Void"),
            Atom::Integer(n) => write!(f, "{}", n),
            Atom::Bool(b) => write!(f, "{}", b),
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Lambda(parameters, expressions) => {
                write!(f, "Lambda(")?;
                for param in parameters {
                    write!(f, "{} ", param)?;
                }
                write!(f, ")")?;
                for expr in expressions {
                    write!(f, " {}", expr)?;
                }
                Ok(())
            }
            Atom::List(list) => {
                write!(f, "(")?;
                for (index, value) in list.iter().enumerate() {
                    if index > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, ")")
            }
        }
    }
}

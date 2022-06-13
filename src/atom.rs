use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Void,
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Symbol(String),
    Lambda(Vec<String>, Vec<Atom>),
    List(Vec<Atom>),
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Void => write!(f, "Void"),
            Atom::Integer(n) => write!(f, "{}", n),
            Atom::Float(n) => write!(f, "{}", n),
            Atom::Bool(b) => write!(f, "{}", b),
            Atom::String(s) => write!(f, "{}", s),
            Atom::Symbol(s) => write!(f, "{}", s),
            Atom::Lambda(parameters, expressions) => {
                write!(f, "Lambda(")?;
                for parameter in parameters {
                    write!(f, "{} ", parameter)?;
                }
                write!(f, ")")?;
                for expression in expressions {
                    write!(f, " {}", expression)?;
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
                write!(f, ")")?;

                Ok(())
            }
        }
    }
}

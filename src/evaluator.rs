use crate::atom::Atom;
use crate::environment::Environment;
use crate::syntax_parser::parse;

use std::cell::RefCell;
use std::rc::Rc;

fn evaluate_binary_op(
    list: &Vec<Atom>,
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for infix operator".to_string());
    }

    let operator = list[0].clone();
    let left_value = evaluate_atom(&list[1].clone(), environment)?;
    let right_value = evaluate_atom(&list[2].clone(), environment)?;

    match operator {
        Atom::Symbol(s) => match s.as_str() {
            "+" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Integer(l + r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Float(l + r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Float((l as f64) + r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Float(l + (r as f64))),
                _ => Err("Invalid types for + operator".to_string()),
            },
            "-" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Integer(l - r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Float(l - r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Float((l as f64) - r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Float(l - (r as f64))),
                _ => Err("Invalid types for - operator".to_string()),
            },
            "*" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Integer(l * r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Float(l * r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Float((l as f64) * r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Float(l * (r as f64))),
                _ => Err("Invalid types for * operator".to_string()),
            },
            "/" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Integer(l / r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Float(l / r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Float((l as f64) / r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Float(l / (r as f64))),
                _ => Err("Invalid types for / operator".to_string()),
            },
            "<" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Bool(l < r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Bool(l < r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Bool((l as f64) < r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Bool(l < (r as f64))),
                _ => Err("Invalid types for < operator".to_string()),
            },
            "<=" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Bool(l <= r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Bool(l <= r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Bool((l as f64) <= r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Bool(l <= (r as f64))),
                _ => Err("Invalid types for + operator".to_string()),
            },
            ">" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Bool(l > r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Bool(l > r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Bool((l as f64) > r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Bool(l > (r as f64))),
                _ => Err("Invalid types for < operator".to_string()),
            },
            ">=" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Bool(l >= r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Bool(l >= r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Bool((l as f64) >= r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Bool(l >= (r as f64))),
                _ => Err("Invalid types for + operator".to_string()),
            },
            "=" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Bool(l == r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Bool(l == r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Bool((l as f64) == r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Bool(l == (r as f64))),
                _ => Err("Invalid types for < operator".to_string()),
            },
            "!=" => match (left_value, right_value) {
                (Atom::Integer(l), Atom::Integer(r)) => Ok(Atom::Bool(l != r)),
                (Atom::Float(l), Atom::Float(r)) => Ok(Atom::Bool(l != r)),
                (Atom::Integer(l), Atom::Float(r)) => Ok(Atom::Bool((l as f64) != r)),
                (Atom::Float(l), Atom::Integer(r)) => Ok(Atom::Bool(l != (r as f64))),
                _ => Err("Invalid types for + operator".to_string()),
            },
            _ => Err(format!("Invalid infix operator: {}", s)),
        },
        _ => Err("Operator must be a symbol".to_string()),
    }
}

fn evaluate_define(
    list: &Vec<Atom>,
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    if list.len() != 3 {
        return Err("Invalid number of arguments for define".to_string());
    }

    let symbol = match &list[1] {
        Atom::Symbol(s) => s.clone(),
        _ => return Err("Invalid define".to_string()),
    };
    let value = evaluate_atom(&list[2], environment)?;

    environment.borrow_mut().set(&symbol, value);
    Ok(Atom::Void)
}

fn evaluate_if(
    list: &Vec<Atom>,
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    if list.len() != 4 {
        return Err("Invalid number of arguments for if statement".to_string());
    }

    let cond_atom = evaluate_atom(&list[1], environment)?;
    let cond = match cond_atom {
        Atom::Bool(b) => b,
        _ => return Err("Condition must be a boolean".to_string()),
    };

    if cond {
        evaluate_atom(&list[2], environment)
    } else {
        evaluate_atom(&list[3], environment)
    }
}

fn evaluate_do(
    list: &Vec<Atom>,
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    if list.len() < 3 {
        return Err("Invalid number of arguments for do statement".to_string());
    }

    let count_atom = evaluate_atom(&list[1], environment)?;
    let count = match count_atom {
        Atom::Integer(n) => n,
        _ => return Err("Condition must be a boolean".to_string()),
    };

    for _ in 0..count {
        evaluate_atom(&list[2], environment)?;
    }

    Ok(Atom::Void)
}

fn evaluate_function_definition(list: &[Atom]) -> Result<Atom, String> {
    let parameters = match &list[1] {
        Atom::List(list) => {
            let mut parameters = Vec::new();
            for param in list {
                match param {
                    Atom::Symbol(s) => parameters.push(s.clone()),
                    _ => return Err("Invalid lambda parameter".to_string()),
                }
            }

            parameters
        }
        _ => return Err("Invalid lambda".to_string()),
    };

    let body = match &list[2] {
        Atom::List(list) => list.clone(),
        _ => return Err("Invalid lambda".to_string()),
    };

    Ok(Atom::Lambda(parameters, body))
}

fn evaluate_function_call(
    symbol: &str,
    list: &[Atom],
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    let lamdba = environment.borrow_mut().get(symbol);
    if lamdba.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }

    let func = lamdba.unwrap();
    match func {
        Atom::Lambda(params, body) => {
            let mut new_env = Rc::new(RefCell::new(Environment::extend(environment.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = evaluate_atom(&list[i + 1], environment)?;
                new_env.borrow_mut().set(param, val);
            }
            let new_body = body;
            evaluate_atom(&Atom::List(new_body), &mut new_env)
        }
        _ => return Err(format!("Not a lambda: {}", symbol)),
    }
}

fn evaluate_symbol(
    symbol: &str,
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    let value = environment.borrow_mut().get(symbol);
    if value.is_none() {
        return Err(format!("Unbound symbol: {}", symbol));
    }

    Ok(value.unwrap())
}

fn evaluate_print(
    list: &Vec<Atom>,
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    if list.len() < 2 {
        return Err("Invalid number of arguments for quote statement".to_string());
    }

    println!("{}", evaluate_atom(&list[1], environment).unwrap());

    Ok(Atom::Void)
}

fn evaluate_quote(list: &Vec<Atom>) -> Result<Atom, String> {
    if list.len() < 2 {
        return Err("Invalid number of arguments for print statement".to_string());
    }

    Ok(list[1].clone())
}

fn evaluate_list(
    list: &Vec<Atom>,
    environment: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    let head = &list[0];
    match head {
        Atom::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | "<=" | ">" | ">=" | "=" | "!=" => {
                evaluate_binary_op(list, environment)
            }
            "define" => evaluate_define(list, environment),
            "if" => evaluate_if(list, environment),
            "do" => evaluate_do(list, environment),
            "lambda" => evaluate_function_definition(list),
            "print" => evaluate_print(list, environment),
            "quote" => evaluate_quote(list),
            _ => evaluate_function_call(s, list, environment),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = evaluate_atom(obj, environment)?;
                match result {
                    Atom::Void => {}
                    _ => new_list.push(result),
                }
            }

            Ok(Atom::List(new_list))
        }
    }
}

fn evaluate_atom(atom: &Atom, environment: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    match atom {
        Atom::Void => Ok(Atom::Void),
        Atom::Integer(i) => Ok(Atom::Integer(*i)),
        Atom::Float(f) => Ok(Atom::Float(*f)),
        Atom::Bool(b) => Ok(Atom::Bool(*b)),
        Atom::String(s) => Ok(Atom::String(s.clone())),
        Atom::Symbol(s) => evaluate_symbol(s, environment),
        Atom::Lambda(_params, _body) => Ok(Atom::Void),
        Atom::List(list) => evaluate_list(list, environment),
    }
}

pub fn evaluate(program: &str, environment: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    let parsed_list = parse(program);

    evaluate_atom(&parsed_list, environment)
}

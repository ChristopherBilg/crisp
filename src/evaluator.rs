use crate::atom::Atom;
use crate::environment::Environment;
use crate::syntax_parser::parse;

use std::cell::RefCell;
use std::rc::Rc;

fn evaluate_binary_op(
    list: &Vec<Atom>,
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for infix operator"));
    }
    let operator = list[0].clone();
    let left = evaluate_obj(&list[1].clone(), env)?;
    let right = evaluate_obj(&list[2].clone(), env)?;
    let left_val = match left {
        Atom::Integer(n) => n,
        _ => return Err(format!("Left operand must be an integer {:?}", left)),
    };
    let right_val = match right {
        Atom::Integer(n) => n,
        _ => return Err(format!("Right operand must be an integer {:?}", right)),
    };
    match operator {
        Atom::Symbol(s) => match s.as_str() {
            "+" => Ok(Atom::Integer(left_val + right_val)),
            "-" => Ok(Atom::Integer(left_val - right_val)),
            "*" => Ok(Atom::Integer(left_val * right_val)),
            "/" => Ok(Atom::Integer(left_val / right_val)),
            "<" => Ok(Atom::Bool(left_val < right_val)),
            ">" => Ok(Atom::Bool(left_val > right_val)),
            "=" => Ok(Atom::Bool(left_val == right_val)),
            "!=" => Ok(Atom::Bool(left_val != right_val)),
            _ => Err(format!("Invalid infix operator: {}", s)),
        },
        _ => Err(format!("Operator must be a symbol")),
    }
}

fn evaluate_define(list: &Vec<Atom>, env: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    if list.len() != 3 {
        return Err(format!("Invalid number of arguments for define"));
    }

    let sym = match &list[1] {
        Atom::Symbol(s) => s.clone(),
        _ => return Err(format!("Invalid define")),
    };
    let val = evaluate_obj(&list[2], env)?;
    env.borrow_mut().set(&sym, val);
    Ok(Atom::Void)
}

fn evaluate_if(list: &Vec<Atom>, env: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    if list.len() != 4 {
        return Err(format!("Invalid number of arguments for if statement"));
    }

    let cond_obj = evaluate_obj(&list[1], env)?;
    let cond = match cond_obj {
        Atom::Bool(b) => b,
        _ => return Err(format!("Condition must be a boolean")),
    };

    if cond == true {
        return evaluate_obj(&list[2], env);
    } else {
        return evaluate_obj(&list[3], env);
    }
}

fn evaluate_function_definition(list: &Vec<Atom>) -> Result<Atom, String> {
    let params = match &list[1] {
        Atom::List(list) => {
            let mut params = Vec::new();
            for param in list {
                match param {
                    Atom::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid lambda parameter")),
                }
            }
            params
        }
        _ => return Err(format!("Invalid lambda")),
    };

    let body = match &list[2] {
        Atom::List(list) => list.clone(),
        _ => return Err(format!("Invalid lambda")),
    };
    Ok(Atom::Lambda(params, body))
}

fn evaluate_function_call(
    s: &str,
    list: &Vec<Atom>,
    env: &mut Rc<RefCell<Environment>>,
) -> Result<Atom, String> {
    let lamdba = env.borrow_mut().get(s);
    if lamdba.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }

    let func = lamdba.unwrap();
    match func {
        Atom::Lambda(params, body) => {
            let mut new_env = Rc::new(RefCell::new(Environment::extend(env.clone())));
            for (i, param) in params.iter().enumerate() {
                let val = evaluate_obj(&list[i + 1], env)?;
                new_env.borrow_mut().set(param, val);
            }
            let new_body = body.clone();
            return evaluate_obj(&Atom::List(new_body), &mut new_env);
        }
        _ => return Err(format!("Not a lambda: {}", s)),
    }
}

fn evaluate_symbol(s: &str, env: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    let val = env.borrow_mut().get(s);
    if val.is_none() {
        return Err(format!("Unbound symbol: {}", s));
    }
    Ok(val.unwrap().clone())
}

fn evaluate_list(list: &Vec<Atom>, env: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    let head = &list[0];
    match head {
        Atom::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                return evaluate_binary_op(&list, env);
            }
            "define" => evaluate_define(&list, env),
            "if" => evaluate_if(&list, env),
            "lambda" => evaluate_function_definition(&list),
            _ => evaluate_function_call(&s, &list, env),
        },
        _ => {
            let mut new_list = Vec::new();
            for obj in list {
                let result = evaluate_obj(obj, env)?;
                match result {
                    Atom::Void => {}
                    _ => new_list.push(result),
                }
            }
            Ok(Atom::List(new_list))
        }
    }
}

fn evaluate_obj(obj: &Atom, env: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    match obj {
        Atom::List(list) => evaluate_list(list, env),
        Atom::Void => Ok(Atom::Void),
        Atom::Lambda(_params, _body) => Ok(Atom::Void),
        Atom::Bool(_) => Ok(obj.clone()),
        Atom::Integer(n) => Ok(Atom::Integer(n.clone())),
        Atom::Symbol(s) => evaluate_symbol(s, env),
    }
}

pub fn evaluate(program: &str, env: &mut Rc<RefCell<Environment>>) -> Result<Atom, String> {
    let parsed_list = parse(program);
    if parsed_list.is_err() {
        return Err(format!("{}", parsed_list.err().unwrap()));
    }
    evaluate_obj(&parsed_list.unwrap(), env)
}

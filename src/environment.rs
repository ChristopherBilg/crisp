use crate::atom::Atom;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    variables: HashMap<String, Atom>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Environment {
        Environment {
            parent: Some(parent),
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Atom> {
        match self.variables.get(key) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|env| env.borrow().get(key).clone()),
        }
    }

    pub fn set(&mut self, key: &str, value: Atom) {
        self.variables.insert(key.to_string(), value);
    }
}

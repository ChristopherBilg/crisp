use crate::atom::Atom;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    vars: HashMap<String, Atom>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Environment {
        Environment {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Atom> {
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: &str, val: Atom) {
        self.vars.insert(name.to_string(), val);
    }
}

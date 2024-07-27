use std::collections::HashMap;
use crate::nil::grammar::Value;

use crate::nil::grammar::{Function};
use crate::nil::errorhandler::Error;

#[derive(Clone)]
pub struct Scope {
    pub var: Vec<HashMap<String, Value>>,
    pub funs: HashMap<String, Function>//fn prot body
}

impl Scope {
    pub fn new() -> Self {
        let var: Vec<HashMap<String, Value>> = vec!(HashMap::new());
        let funs: HashMap<String, Function> = HashMap::new();

        Scope {var: var, funs: funs}
    }

    pub fn get_var(&self, name: &str) -> Option<Value> {
        //println!("get: {:?}", &self.var);
        for i in (0..self.var.len()).rev() {
            match self.var[i].get(name) {
                Some(val) => return Some(val.clone()),
                None => {}
            }
        }
        None
    }

    pub fn set_var_local(&mut self, name: String, val: Value) -> Result<Value, Error> {
        //println!("set: {:?}", &self.var);
        let depth = self.var.len()-1;
        self.var[depth].insert(name, val);

        return Ok(Value::Bool(true))
    }

    pub fn set_var(&mut self, name: String, val: Value) -> Result<Value, Error> {
        //println!("set: {:?}", &self.var);
        for i in (0..self.var.len()).rev() {
            match self.var[i].get(&name) {
                Some(_) => {
                    self.var[i].insert(name, val);
                    return Ok(Value::Bool(true));
                },
                None => {}
            }
        }

        let depth = self.var.len()-1;
        self.var[depth].insert(name, val);

        return Ok(Value::Bool(true))
    }

    pub fn create_depth(&mut self) -> Result<(), Error> {
        //println!("str: {:?}", &self.var);
        self.var.push(HashMap::new());
        //println!("end: {:?}", &self.var);
        Ok(())
    }

    pub fn remove_depth(&mut self) -> Result<(), Error> {
        self.var.pop();
        Ok(())
    }
}

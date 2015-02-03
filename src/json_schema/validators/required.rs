use rustc_serialize::json;

use super::super::errors;
use super::super::scope;

#[allow(missing_copy_implementations)]
pub struct Required {
    pub items: Vec<String>
}

impl super::Validator for Required {
    fn validate(&self, val: &json::Json, path: &str, strict: bool, _scope: &scope::Scope) -> super::ValidationState {
        let object = strict_process!(val.as_object(), path, strict, "The value must be an object");
        let mut state = super::ValidationState::new();

        for key in self.items.iter() {
            if !object.contains_key(key) {
                state.errors.push(Box::new(
                    errors::Required {
                        path: [path, key.as_slice()].connect("/")
                    }
                ))          
            }
        }

        state
    }
}
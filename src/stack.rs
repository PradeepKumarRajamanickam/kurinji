use std::collections::HashMap;

use crate::{config::Config, inputmap::InputMap};

impl InputMap {
    pub fn push(&mut self, config: Config) {
        // store current in stack
        let current = self.get_binding();
        self.stack.push(current);

        // set new config as current
        self.set_bindings(config);
    }

    pub fn push_additive(&mut self, config: Config) {
        
        // store current in stack
        let current = self.get_binding();
        self.stack.push(current);

        // additive merge
        let mut new_binding = self.stack.last()
        .unwrap()
        .clone();
        new_binding.merge(config);

        // set new config as current
        self.set_bindings(new_binding);
    }

    pub fn pop(&mut self) {
        if let Some(next) = self.stack.pop() {
            self.set_bindings(next);
        }
    }
}

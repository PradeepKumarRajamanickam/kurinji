use std::collections::HashMap;

use crate::{bindings::Bindings, inputmap::InputMap};

impl InputMap {
    // Push bindings into stack
    pub fn push(&mut self, bindings: Bindings) {
        // store current in stack
        let current = self.get_bindings();
        self.stack.push(current);

        // set new config as current
        self.set_bindings(bindings);
    }

    /// Clones current bindings updates it with the pushed
    /// config
    pub fn push_additive(&mut self, bindings: Bindings) {
        
        // store current in stack
        let current = self.get_bindings();
        self.stack.push(current);

        // additive merge
        let mut new_binding = self.stack.last()
        .unwrap()
        .clone();
        new_binding.merge(bindings);

        // set new config as current
        self.set_bindings(new_binding);
    }

    /// Pop the current binding out of stack
    pub fn pop(&mut self) {
        if let Some(next) = self.stack.pop() {
            self.set_bindings(next);
        }
    }
}

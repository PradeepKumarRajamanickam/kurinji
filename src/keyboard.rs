use crate::inputmap::InputMap;
use bevy_ecs::{Res, ResMut};
use bevy_input::{prelude::KeyCode, Input};
use std::collections::HashMap;

impl InputMap {
    // publics
    pub fn bind_keyboard_pressed(&mut self, code: KeyCode, action: &str) {
        self.keyboard_action_binding.insert(code, action.to_string());
    }

    pub fn unbind_keyboard_pressed(&mut self, code: KeyCode) {
        self.keyboard_action_binding.remove(&code);
    }

    // crates
    pub(crate) fn set_bindings(&mut self, binding: HashMap<KeyCode, String>)
    {
        self.keyboard_action_binding = binding;
    }
    pub(crate) fn get_bindings(&self) -> HashMap<KeyCode, String>
    {
        self.keyboard_action_binding.clone()
    }

    // system
    pub(crate) fn kb_key_press_input_system(
        mut input_map: ResMut<InputMap>,
        key_input: Res<Input<KeyCode>>,
    ) {
        // let map = &mut input_map;
        let bindings_iter = input_map.keyboard_action_binding.clone();

        for (keycode, action) in bindings_iter.iter() {
            if key_input.pressed(*keycode) {
                input_map.set_raw_action_strength(action, 1.0);
            }
        }
    }
}

use crate::input_map::InputMap;
use bevy::prelude::KeyCode;
use bevy_ecs::{Res, ResMut};
use bevy_input::Input;

impl InputMap {
    // publics
    pub fn bind_keyboard_pressed(&mut self, code: KeyCode, action: &str) -> &mut InputMap {
        self.keyboard_action_binding
            .insert(code, action.to_string());
        self
    }

    pub fn unbind_keyboard_pressed(&mut self, code: KeyCode) -> &mut InputMap {
        self.keyboard_action_binding.remove(&code);
        self
    }

    // crates
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

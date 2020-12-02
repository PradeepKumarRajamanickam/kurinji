use crate::{
    kurinji::Kurinji,
    Actionable,
};

use bevy::prelude::KeyCode;
use bevy::ecs::{Res, ResMut};
use bevy::input::Input;

impl<T: Actionable> Kurinji<T> {
    // publics
    pub fn bind_keyboard_pressed(&mut self, code: KeyCode, action: T) -> &mut Kurinji<T> {
        self.keyboard_action_binding
            .insert(code, action);
        self
    }

    pub fn unbind_keyboard_pressed(&mut self, code: KeyCode) -> &mut Kurinji<T> {
        self.keyboard_action_binding.remove(&code);
        self
    }

    // crates
    // system
    pub(crate) fn kb_key_press_input_system(
        mut input_map: ResMut<Kurinji<T>>,
        key_input: Res<Input<KeyCode>>,
    ) {
        // let map = &mut input_map;
        let mut action_strengths = Vec::new();

        for (keycode, action) in input_map.keyboard_action_binding.iter() {
            if key_input.pressed(*keycode) {
                action_strengths.push(*action);
            }
        }

        for action in action_strengths.drain(..){
            input_map.set_raw_action_strength(action, 1.0);
        }
    }
}

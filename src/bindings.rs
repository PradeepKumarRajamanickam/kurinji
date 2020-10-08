use crate::{EventPhase, InputMap, axis::MouseAxis, axis::GamepadAxis};

use bevy::prelude::*;
use std::{collections::HashMap, hash::Hash};

/// Data structure to hold bindings.
#[derive(Default, Clone, Debug)]
pub struct Bindings {
    pub(crate) gamepad_button_bindings: HashMap<(usize, GamepadButtonType), String>,
    pub(crate) gamepad_axis_bindings: HashMap<(usize, GamepadAxis), String>,
    pub(crate) keyboard_key_bindings: HashMap<KeyCode, String>,
    pub(crate) mouse_button_binding: HashMap<MouseButton, String>,
    pub(crate) mouse_move_binding: HashMap<MouseAxis, String>,
    pub(crate) action_deadzone: HashMap<String, f32>,
    pub(crate) action_phase: HashMap<String, EventPhase>,
}

impl Bindings {
    // publics
    pub fn merge(&mut self, bindings: Bindings) {
        // keyboard
        self.keyboard_key_bindings = Bindings::get_merged_hashmaps(
            self.keyboard_key_bindings.clone(),
            bindings.keyboard_key_bindings,
        );

        // mouse
        self.mouse_button_binding = Bindings::get_merged_hashmaps(
            self.mouse_button_binding.clone(),
            bindings.mouse_button_binding,
        );
        self.mouse_move_binding = Bindings::get_merged_hashmaps(
            self.mouse_move_binding.clone(),
            bindings.mouse_move_binding,
        );

        // actions
        self.action_deadzone =
            Bindings::get_merged_hashmaps(self.action_deadzone.clone(), bindings.action_deadzone);
        self.action_phase =
            Bindings::get_merged_hashmaps(self.action_phase.clone(), bindings.action_phase);
    }

    // private
    // src: https://stackoverflow.com/questions/27244465/merge-two-hashmaps-in-rust
    fn get_merged_hashmaps<K: Hash + Eq + Clone, V: Clone>(
        map1: HashMap<K, V>,
        map2: HashMap<K, V>,
    ) -> HashMap<K, V> {
        map1.into_iter().chain(map2).collect()
    }
}

impl InputMap {
    // public
    pub fn get_bindings(&self) -> Bindings {
        Bindings {
            gamepad_button_bindings: self.joystick_button_binding.clone(),
            gamepad_axis_bindings: self.joystick_axis_binding.clone(),
            keyboard_key_bindings: self.keyboard_action_binding.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),

            action_deadzone: self.action_deadzone.clone(),
            action_phase: self.action_phase.clone(),
        }
    }
    pub fn set_bindings(&mut self, bindings: Bindings) {
        self.joystick_button_binding = bindings.gamepad_button_bindings;
        self.joystick_axis_binding = bindings.gamepad_axis_bindings;
        self.keyboard_action_binding = bindings.keyboard_key_bindings;
        self.mouse_button_binding = bindings.mouse_button_binding;
        self.mouse_move_binding = bindings.mouse_move_binding;
        self.action_deadzone = bindings.action_deadzone;
        self.action_phase = bindings.action_phase;
    }
}

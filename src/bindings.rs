use bevy::prelude::{KeyCode, MouseButton, GamepadButton};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

use crate::{axis::Axis, inputmap::InputMap, eventphase::EventPhase};

/// Data structure for serde
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bindings {
    #[serde(default, rename = "GamepadButtons")]
    gamepad_button_bindings: HashMap<GamepadButton, String>,
    #[serde(default, rename = "KeyboardKeys")]
    keyboard_key_bindings: HashMap<KeyCode, String>,
    #[serde(default, rename = "MouseButtons")]
    mouse_button_binding: HashMap<MouseButton, String>,
    #[serde(default, rename = "MouseMove")]
    mouse_move_binding: HashMap<Axis, String>,
    #[serde(default, rename = "DeadZone")]
    action_deadzone: HashMap<String, f32>,
    #[serde(default, rename = "EventPhase")]
    action_phase: HashMap<String, EventPhase>,
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
        self.action_deadzone = Bindings::get_merged_hashmaps(
            self.action_deadzone.clone(),
            bindings.action_deadzone,
        );
        self.action_phase = Bindings::get_merged_hashmaps(
            self.action_phase.clone(),
            bindings.action_phase,
        );
    }

    // private
    // src: https://stackoverflow.com/questions/27244465/merge-two-hashmaps-in-rust
    fn get_merged_hashmaps<K: Hash + Eq + Clone, V: Clone>(
        map1: HashMap<K, V>,
        map2: HashMap<K, V>,
    ) -> HashMap<K, V> {
        map1.clone().into_iter().chain(map2).collect()
    }
}

impl InputMap {
    // public
    pub fn get_bindings(&self) -> Bindings {
        Bindings {
            gamepad_button_bindings: self.joystick_button_binding.clone(),
            keyboard_key_bindings: self.keyboard_action_binding.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),

            action_deadzone: self.action_deadzone.clone(),
            action_phase: self.action_phase.clone()
        }
    }
    pub fn set_bindings(&mut self, bindings: Bindings) {
        self.joystick_button_binding = bindings.gamepad_button_bindings;
        self.keyboard_action_binding = bindings.keyboard_key_bindings;
        self.mouse_button_binding = bindings.mouse_button_binding;
        self.mouse_move_binding = bindings.mouse_move_binding;
        self.action_deadzone = bindings.action_deadzone;
        self.action_phase = bindings.action_phase;
    }
}

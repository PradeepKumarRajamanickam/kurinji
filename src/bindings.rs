use bevy::prelude::{KeyCode, MouseButton};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

use crate::{axis::Axis, inputmap::InputMap, util, eventphase::EventPhase};

/// Data structure for serde
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Bindings {
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
            keyboard_key_bindings: self.keyboard_action_binding.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),

            action_deadzone: self.action_deadzone.clone(),
            action_phase: self.action_phase.clone()
        }
    }
    pub fn set_bindings(&mut self, bindings: Bindings) {
        self.keyboard_action_binding = bindings.keyboard_key_bindings;
        self.mouse_button_binding = bindings.mouse_button_binding;
        self.mouse_move_binding = bindings.mouse_move_binding;
        self.action_deadzone = bindings.action_deadzone;
        self.action_phase = bindings.action_phase;
    }

    // ron
    pub fn get_bindings_as_ron(&self) -> Result<String, String> {
        let data = self.get_bindings();
        let pretty = ron::ser::PrettyConfig::new()
            .with_enumerate_arrays(true)
            .with_new_line("\n".to_string());
        let serialized = ron::ser::to_string_pretty(&data, pretty);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err("Failed to generate ron".to_string()),
        }
    }
    pub fn set_bindings_with_ron(&mut self, ron: &str) {
        let bindings: Bindings = util::get_bindings_from_ron(ron);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
    }

    // json
    pub fn get_bindings_as_json(&self) -> Result<String, String> {
        let data = self.get_bindings();
        let serialized = serde_json::to_string_pretty(&data);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err("Failed to generate json".to_string()),
        }
    }
    pub fn set_bindings_with_json(&mut self, json: &str) {
        let bindings: Bindings = util::get_bindings_from_json(json);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
    }
}

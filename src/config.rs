use bevy::prelude::{KeyCode, MouseButton};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash};

use crate::{axis::Axis, inputmap::InputMap, util};

/// Data structure for serde
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, rename = "KeyboardKeys")]
    keyboard_key_bindings: HashMap<KeyCode, String>,
    #[serde(default, rename = "MouseButtons")]
    mouse_button_binding: HashMap<MouseButton, String>,
    #[serde(default, rename = "MouseMove")]
    mouse_move_binding: HashMap<Axis, String>,
    #[serde(default, rename = "DeadZone")]
    action_deadzone: HashMap<String, f32>,
}

impl Config {
    // publics
    pub fn merge(&mut self, config: Config) {

        // keyboard
        self.keyboard_key_bindings = Config::get_merged_hashmaps(
            config.keyboard_key_bindings,
            self.keyboard_key_bindings.clone(),
        );

        // mouse
        self.mouse_button_binding = Config::get_merged_hashmaps(
            config.mouse_button_binding,
            self.mouse_button_binding.clone(),
        );
        self.mouse_move_binding = Config::get_merged_hashmaps(
            config.mouse_move_binding,
            self.mouse_move_binding.clone(),
        );

        // actions
        self.action_deadzone = Config::get_merged_hashmaps(
            config.action_deadzone,
            self.action_deadzone.clone(),
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
    pub fn get_bindings(&self) -> Config {
        Config {
            keyboard_key_bindings: self.keyboard_action_binding.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),

            action_deadzone: self.action_deadzone.clone(),
        }
    }
    pub fn set_bindings(&mut self, config: Config) {
        self.keyboard_action_binding = config.keyboard_key_bindings;
        self.mouse_button_binding = config.mouse_button_binding;
        self.mouse_move_binding = config.mouse_move_binding;
        self.action_deadzone = config.action_deadzone;
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
        let config: Config = util::get_config_from_ron(ron);
        self.set_bindings(config);

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
        let config: Config = util::get_config_from_json(json);
        self.set_bindings(config);

        self.action_strength_curve.clear();
    }
}

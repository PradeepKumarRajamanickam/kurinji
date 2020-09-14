use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bevy::prelude::{KeyCode, MouseButton};

use crate::{axis::Axis, inputmap::InputMap};

/// Data structure for serde
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ConfigData
{
    #[serde(rename = "KeyboardKeys")]
    keyboard_key_bindings: HashMap<KeyCode, String>,
    #[serde(rename = "MouseButtons")]
    mouse_button_binding: HashMap<MouseButton, String>,
    #[serde(rename = "MouseMove")]
    mouse_move_binding: HashMap<Axis, String>,
    #[serde(rename = "DeadZone")]
    action_deadzone: HashMap<String, f32>
}

impl InputMap{

    // public
    pub fn get_bindings_as_ron(&self) ->  Result<String, String>
    {
        let data = ConfigData
        {
            keyboard_key_bindings: self.keyboard_action_binding.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),

            action_deadzone: self.action_deadzone.clone()
        };
        let pretty = ron::ser::PrettyConfig::new()
        .with_enumerate_arrays(true)
        .with_new_line("\n".to_string());
        let serialized = ron::ser::to_string_pretty(&data, pretty);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err("Failed to generate ron".to_string()),
        }
    }

    pub fn set_bindings_with_ron(&mut self, ron: &str)
    {
        let config: ConfigData = ron::de::from_str(ron)
        .expect("Failed to deserialise config ron");

        self.keyboard_action_binding = config.keyboard_key_bindings;
        self.mouse_button_binding = config.mouse_button_binding;
        self.mouse_move_binding = config.mouse_move_binding;
        self.action_deadzone = config.action_deadzone;

        self.action_strength_curve.clear();
    } 

    pub fn get_bindings_as_json(&self) ->  Result<String, String>
    {
        let data = ConfigData
        {
            keyboard_key_bindings: self.keyboard_action_binding.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),

            action_deadzone: self.action_deadzone.clone()
        };
        let serialized = serde_json::to_string_pretty(&data);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err("Failed to generate json".to_string()),
        }
    }

    pub fn set_bindings_with_json(&mut self, json: &str)
    {
        let config: ConfigData = serde_json::from_str(json)
        .expect("Failed to deserialise config json");

        self.keyboard_action_binding = config.keyboard_key_bindings;
        self.mouse_button_binding = config.mouse_button_binding;
        self.mouse_move_binding = config.mouse_move_binding;
        self.action_deadzone = config.action_deadzone;

        self.action_strength_curve.clear();
    } 
}
use crate::{MouseAxis, Bindings, EventPhase, Kurinji, axis::GamepadAxis, Actionable};

use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

impl<T: Actionable> Kurinji<T> {
    // publics
    // ron
    pub fn get_bindings_as_ron(&self) -> Result<String, String> {
        let data = self.get_bindings();
        let pretty = ron::ser::PrettyConfig::new()
            .with_enumerate_arrays(true)
            .with_new_line("\n".to_string());
        let serialized = ron::ser::to_string_pretty(&data, pretty);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Failed to generate ron {}", e)),
        }
    }
    pub fn set_bindings_with_ron(&mut self, ron: &str) -> &mut Kurinji<T>  {
        let bindings: Bindings<T> = Kurinji::<T>::get_bindings_from_ron(ron);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
        self
    }

    // json
    pub fn get_bindings_as_json(&self) -> Result<String, String> {
        let data = self.get_bindings();
        let serialized = serde_json::to_string_pretty(&data);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Failed to generate json {}", e)),
        }
    }

    pub fn set_bindings_with_json(&mut self, json: &str)  -> &mut Kurinji<T> {
        let bindings: Bindings<T> = Kurinji::<T>::get_bindings_from_json(json);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
        self
    }
}

#[derive(Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BindingsSerdeHelper<T: std::hash::Hash + Eq + PartialEq> {
    #[serde(default, rename = "GamepadButtons")]
    gamepad_button_bindings: HashMap<usize, HashMap<GamepadButtonType, T>>,
    #[serde(default, rename = "GamepadAxis")]
    gamepad_axis_bindings: HashMap<usize, HashMap<GamepadAxis, T>>,

    #[serde(default, rename = "KeyboardKeys")]
    keyboard_key_bindings: HashMap<KeyCode, T>,
    #[serde(default, rename = "MouseButtons")]
    mouse_button_binding: HashMap<MouseButton, T>,
    #[serde(default, rename = "MouseMove")]
    mouse_move_binding: HashMap<MouseAxis, T>,

    #[serde(default, rename = "DeadZone")]
    action_deadzone: HashMap<T, f32>,

    #[serde(default, rename = "EventPhase")]
    action_phase: HashMap<T, EventPhase>,
}

impl<T: Actionable> Serialize for Bindings<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BindingsSerdeHelper {
            keyboard_key_bindings: self.keyboard_key_bindings.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),
            action_deadzone: self.action_deadzone.clone(),
            action_phase: self.action_phase.clone(),
            gamepad_button_bindings: BindingsSerdeHelper::get_json_friendly_gamepad_button_hash_map(
                self.gamepad_button_bindings.clone(),
            ),
            gamepad_axis_bindings: BindingsSerdeHelper::get_json_friendly_gamepad_axis_hash_map(
                self.gamepad_axis_bindings.clone(),
            ),
        }
        .serialize(serializer)
    }
}

impl<'de, T: Actionable> Deserialize<'de> for Bindings<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(
            |BindingsSerdeHelper {
                 gamepad_button_bindings,
                 gamepad_axis_bindings,
                 keyboard_key_bindings,
                 mouse_button_binding,
                 mouse_move_binding,
                 action_deadzone,
                 action_phase,
             }| Bindings {
                keyboard_key_bindings,
                mouse_button_binding,
                mouse_move_binding,
                action_deadzone,
                action_phase,

                gamepad_button_bindings:
                    BindingsSerdeHelper::get_gamepad_button_hash_map_from_json_friendly_map(
                        gamepad_button_bindings,
                    ),
                gamepad_axis_bindings:
                    BindingsSerdeHelper::get_gamepad_axis_hash_map_from_json_friendly_map(
                        gamepad_axis_bindings,
                    ),
            },
        )
    }
}

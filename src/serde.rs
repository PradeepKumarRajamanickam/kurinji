use bevy::prelude::{
    Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, KeyCode, MouseButton,
};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

// TODO Implement Deserialize Gamepad Bindings
use crate::{
    axis::{AnalogDirection, Axis},
    bindings::Bindings,
    eventphase::EventPhase,
    gamepad::GamepadAnalog,
    inputmap::InputMap,
};

impl InputMap {
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
    pub fn set_bindings_with_ron(&mut self, ron: &str) {
        let bindings: Bindings = InputMap::get_bindings_from_ron(ron);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
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
    pub fn set_bindings_with_json(&mut self, json: &str) {
        let bindings: Bindings = InputMap::get_bindings_from_json(json);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GamepadAxisHelper {
    LeftStickXPositive,
    LeftStickXNegative,

    LeftStickYPositive,
    LeftStickYNegative,

    LeftZPositive,
    LeftZNegative,

    RightStickXPositive,
    RightStickXNegative,

    RightStickYPositive,
    RightStickYNegative,

    RightZPositive,
    RightZNegative,

    DPadXPositive,
    DPadXNegative,

    DPadYPositive,
    DPadYNegative,
}

impl Default for GamepadAxisHelper {
    fn default() -> Self {
        GamepadAxisHelper::LeftStickXPositive
    }
}

#[derive(Default, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BindingsSerdeHelper {
    #[serde(default, rename = "GamepadButtons")]
    gamepad_button_bindings: HashMap<usize, HashMap<GamepadButtonType, String>>,
    #[serde(default, rename = "GamepadAnalogs")]
    gamepad_axis_bindings: HashMap<usize, HashMap<GamepadAxisHelper, String>>,

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
impl BindingsSerdeHelper {
    // utils
    pub fn get_gamepad_button_hash_map_from_json_friendly_map(
        json_map: HashMap<usize, HashMap<GamepadButtonType, String>>,
    ) -> HashMap<GamepadButton, String> {
        let mut result: HashMap<GamepadButton, String> = HashMap::new();
        for (pad_handle, h_map) in json_map {
            for (btn_type, action) in h_map {
                result
                    .entry(GamepadButton(Gamepad(pad_handle), btn_type))
                    .or_insert(action);
            }
        }
        result
    }
    pub fn get_gamepad_analog_hash_map_from_json_friendly_map(
        json_map: HashMap<usize, HashMap<GamepadAxisHelper, String>>,
    ) -> HashMap<GamepadAnalog, String> {
        let mut result: HashMap<GamepadAnalog, String> = HashMap::new();
        for (pad_handle, h_map) in json_map {
            for (axis_helper, action) in h_map {
                let analog =
                    BindingsSerdeHelper::get_gamepad_analog_from_axis_helper(pad_handle, axis_helper);
                result.entry(analog).or_insert(action);
            }
        }
        result
    }
    pub fn get_json_friendly_gamepad_button_hash_map(
        binding: HashMap<GamepadButton, String>,
    ) -> HashMap<usize, HashMap<GamepadButtonType, String>> {
        let mut result: HashMap<usize, HashMap<GamepadButtonType, String>> = HashMap::new();

        for (k, v) in binding {
            let id: usize = (k.0).0;
            let button = k.1;
            let action = v;

            result.entry(id).or_insert_with(HashMap::new);
            result.get_mut(&id).unwrap().insert(button, action);
        }
        result
    }
    pub fn get_json_friendly_gamepad_analog_hash_map(
        binding: HashMap<GamepadAnalog, String>,
    ) -> HashMap<usize, HashMap<GamepadAxisHelper, String>> {
        let mut result: HashMap<usize, HashMap<GamepadAxisHelper, String>> = HashMap::new();

        for (k, action) in binding {
            let id = (k.axis.0).0;
            let axis_helper = BindingsSerdeHelper::get_gamepad_axis_helper(k);

            result.entry(id).or_insert_with(HashMap::new);
            result.get_mut(&id).unwrap().insert(axis_helper, action);
        }
        result
    }

    pub fn get_gamepad_analog_from_axis_helper(
        pad_handle: usize,
        axis_helper: GamepadAxisHelper,
    ) -> GamepadAnalog {
        match axis_helper {
            GamepadAxisHelper::LeftStickXPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::LeftStickX),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::LeftStickXNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::LeftStickX),
                direction: AnalogDirection::Negative,
            },

            GamepadAxisHelper::LeftStickYPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::LeftStickY),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::LeftStickYNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::LeftStickY),
                direction: AnalogDirection::Negative,
            },

            GamepadAxisHelper::LeftZPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::LeftZ),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::LeftZNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::LeftZ),
                direction: AnalogDirection::Negative,
            },

            GamepadAxisHelper::RightStickXPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::RightStickX),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::RightStickXNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::RightStickX),
                direction: AnalogDirection::Negative,
            },

            GamepadAxisHelper::RightStickYPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::RightStickY),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::RightStickYNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::RightStickY),
                direction: AnalogDirection::Negative,
            },

            GamepadAxisHelper::RightZPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::RightZ),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::RightZNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::RightZ),
                direction: AnalogDirection::Negative,
            },

            GamepadAxisHelper::DPadXPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::DPadX),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::DPadXNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::DPadX),
                direction: AnalogDirection::Negative,
            },

            GamepadAxisHelper::DPadYPositive => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::DPadY),
                direction: AnalogDirection::Positve,
            },
            GamepadAxisHelper::DPadYNegative => GamepadAnalog {
                axis: GamepadAxis(Gamepad(pad_handle), GamepadAxisType::DPadY),
                direction: AnalogDirection::Negative,
            },
        }
    }

    pub fn get_gamepad_axis_helper(analog: GamepadAnalog) -> GamepadAxisHelper {
        let game_axis_type = analog.axis.1;
        let direction = analog.direction;

        match (game_axis_type, direction) {
            (GamepadAxisType::LeftStickX, AnalogDirection::Positve) => {
                GamepadAxisHelper::LeftStickXPositive
            }
            (GamepadAxisType::LeftStickX, AnalogDirection::Negative) => {
                GamepadAxisHelper::LeftStickXNegative
            }

            (GamepadAxisType::LeftStickY, AnalogDirection::Positve) => {
                GamepadAxisHelper::LeftStickYPositive
            }
            (GamepadAxisType::LeftStickY, AnalogDirection::Negative) => {
                GamepadAxisHelper::LeftStickYNegative
            }

            (GamepadAxisType::LeftZ, AnalogDirection::Positve) => GamepadAxisHelper::LeftZPositive,
            (GamepadAxisType::LeftZ, AnalogDirection::Negative) => GamepadAxisHelper::LeftZNegative,

            (GamepadAxisType::RightStickX, AnalogDirection::Positve) => {
                GamepadAxisHelper::RightStickXPositive
            }
            (GamepadAxisType::RightStickX, AnalogDirection::Negative) => {
                GamepadAxisHelper::RightStickXNegative
            }

            (GamepadAxisType::RightStickY, AnalogDirection::Positve) => {
                GamepadAxisHelper::RightStickYPositive
            }
            (GamepadAxisType::RightStickY, AnalogDirection::Negative) => {
                GamepadAxisHelper::RightStickYNegative
            }

            (GamepadAxisType::RightZ, AnalogDirection::Positve) => {
                GamepadAxisHelper::RightZPositive
            }
            (GamepadAxisType::RightZ, AnalogDirection::Negative) => {
                GamepadAxisHelper::RightZNegative
            }

            (GamepadAxisType::DPadX, AnalogDirection::Positve) => GamepadAxisHelper::DPadXPositive,
            (GamepadAxisType::DPadX, AnalogDirection::Negative) => GamepadAxisHelper::DPadXNegative,

            (GamepadAxisType::DPadY, AnalogDirection::Positve) => GamepadAxisHelper::DPadYPositive,
            (GamepadAxisType::DPadY, AnalogDirection::Negative) => GamepadAxisHelper::DPadYNegative,
        }
    }
}
impl Serialize for Bindings {
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
            gamepad_axis_bindings: BindingsSerdeHelper::get_json_friendly_gamepad_analog_hash_map(
                self.gamepad_axis_bindings.clone(),
            ),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bindings {
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
                    BindingsSerdeHelper::get_gamepad_analog_hash_map_from_json_friendly_map(
                        gamepad_axis_bindings,
                    ),
            },
        )
    }
}

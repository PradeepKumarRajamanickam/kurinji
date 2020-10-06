use bevy::prelude::{
    Gamepad, GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, Vec2,
};

use std::{collections::HashMap, fs};

use crate::{
    axis::AnalogDirection, bindings::Bindings, gamepad::GamepadAnalog, InputMap,
    serde::BindingsSerdeHelper, serde::GamepadAxisHelper,
};

impl InputMap {
    // publics
    pub fn get_bindings_from_json_file(path: &str) -> Bindings {
        let json = fs::read_to_string(path).expect("Error! could not open bindings file");
        InputMap::get_bindings_from_json(&json)
    }

    pub fn get_bindings_from_ron_file(path: &str) -> Bindings {
        let ron = fs::read_to_string(path).expect("Error! could not open bindings file");
        InputMap::get_bindings_from_ron(&ron)
    }

    pub fn get_bindings_from_json(json: &str) -> Bindings {
        serde_json::from_str(json).expect("Failed to deserialise bindings json")
    }
    pub fn get_bindings_from_ron(ron: &str) -> Bindings {
        ron::de::from_str(ron).expect("Failed to deserialise bindings ron")
    }
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
                let analog = BindingsSerdeHelper::get_gamepad_analog_from_axis_helper(
                    pad_handle,
                    axis_helper,
                );
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

pub(crate) fn normalised_within_range(min: f32, max: f32, value: f32) -> f32 {
    // src: https://stats.stackexchange.com/questions/70801/how-to-normalize-data-to-0-1-range
    // formula: newvalue= (max'-min')/(max-min)*(value-max)+max'
    // new arbitrary range min' to max'
    let r = 1. / (max - min) * (value - max) + 1.0;
    clamp(0., 1., r)
}

pub(crate) fn clamp(min: f32, max: f32, value: f32) -> f32 {
    if value < min {
        return min;
    } else if value > max {
        return max;
    }
    value
}

pub(crate) fn clamp_vec2(min: Vec2, max: Vec2, value: Vec2) -> Vec2 {
    Vec2::new(
        clamp(min.x(), max.x(), value.x()),
        clamp(min.y(), max.y(), value.y()),
    )
}

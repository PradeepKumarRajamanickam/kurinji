use crate::{GamepadAxis, InputMap, bindings::Bindings, serde::BindingsSerdeHelper};

use bevy::prelude::*;
use std::{collections::HashMap, fs};

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

    // crate
    pub(crate) fn is_gamepad_axis_positive(axis: GamepadAxis) -> bool
    {
        return axis == GamepadAxis::LeftStickXPositive
        || axis == GamepadAxis::LeftStickYPositive
        || axis == GamepadAxis::LeftZPositive
        || axis == GamepadAxis::RightStickXPositive
        || axis == GamepadAxis::RightStickYPositive
        || axis == GamepadAxis::RightZPositive
        || axis == GamepadAxis::DPadXPositive
        || axis == GamepadAxis::DPadYPositive;
    }

    pub(crate) fn get_bevy_gamepad_axis_type_from_pad_axis(
        axis: GamepadAxis,
    ) -> bevy::input::gamepad::GamepadAxisType {
        match axis {
            GamepadAxis::LeftStickXPositive => bevy::input::gamepad::GamepadAxisType::LeftStickX,
            GamepadAxis::LeftStickXNegative => bevy::input::gamepad::GamepadAxisType::LeftStickX,

            GamepadAxis::LeftStickYPositive => bevy::input::gamepad::GamepadAxisType::LeftStickY,
            GamepadAxis::LeftStickYNegative => bevy::input::gamepad::GamepadAxisType::LeftStickY,

            GamepadAxis::LeftZPositive => bevy::input::gamepad::GamepadAxisType::LeftZ,
            GamepadAxis::LeftZNegative => bevy::input::gamepad::GamepadAxisType::LeftZ,

            GamepadAxis::RightStickXPositive => bevy::input::gamepad::GamepadAxisType::RightStickX,
            GamepadAxis::RightStickXNegative => bevy::input::gamepad::GamepadAxisType::RightStickX,

            GamepadAxis::RightStickYPositive => bevy::input::gamepad::GamepadAxisType::RightStickY,
            GamepadAxis::RightStickYNegative => bevy::input::gamepad::GamepadAxisType::RightStickY,

            GamepadAxis::RightZPositive => bevy::input::gamepad::GamepadAxisType::RightZ,
            GamepadAxis::RightZNegative => bevy::input::gamepad::GamepadAxisType::RightZ,

            GamepadAxis::DPadXPositive => bevy::input::gamepad::GamepadAxisType::DPadX,
            GamepadAxis::DPadXNegative => bevy::input::gamepad::GamepadAxisType::DPadX,

            GamepadAxis::DPadYPositive => bevy::input::gamepad::GamepadAxisType::DPadY,
            GamepadAxis::DPadYNegative => bevy::input::gamepad::GamepadAxisType::DPadY,
        }
    } 
}

impl BindingsSerdeHelper {
    // utils
    pub fn get_gamepad_button_hash_map_from_json_friendly_map(
        json_map: HashMap<usize, HashMap<GamepadButtonType, String>>,
    ) -> HashMap<(usize, GamepadButtonType), String> {
        let mut result: HashMap<(usize, GamepadButtonType), String>  = HashMap::new();
        for (player, h_map) in json_map {
            for (btn_type, action) in h_map {
                result
                    .entry((player, btn_type))
                    .or_insert(action);
            }
        }
        result
    }
    pub fn get_gamepad_axis_hash_map_from_json_friendly_map(
        json_map: HashMap<usize, HashMap<GamepadAxis, String>>,
    ) -> HashMap<(usize, GamepadAxis), String> {
        let mut result: HashMap<(usize, GamepadAxis), String> = HashMap::new();
        for (pad_handle, h_map) in json_map {
            for (g_axis, action) in h_map {
                result.entry((pad_handle, g_axis)).or_insert(action);
            }
        }
        result
    }
    pub fn get_json_friendly_gamepad_button_hash_map(
        binding: HashMap<(usize, GamepadButtonType), String>,
    ) -> HashMap<usize, HashMap<GamepadButtonType, String>> {
        let mut result: HashMap<usize, HashMap<GamepadButtonType, String>> = HashMap::new();

        for (k, v) in binding {
            let id: usize = k.0;
            let button = k.1;
            let action = v;

            result.entry(id).or_insert_with(HashMap::new);
            result.get_mut(&id).unwrap().insert(button, action);
        }
        result
    }
    pub fn get_json_friendly_gamepad_axis_hash_map(
        binding: HashMap<(usize, GamepadAxis), String>,
    ) -> HashMap<usize, HashMap<GamepadAxis, String>> {
        let mut result: HashMap<usize, HashMap<GamepadAxis, String>> = HashMap::new();

        for (k, action) in binding {
            let id = k.0;
            let axis_helper = k.1;

            result.entry(id).or_insert_with(HashMap::new);
            result.get_mut(&id).unwrap().insert(axis_helper, action);
        }
        result
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

use crate::{
    GamepadAxis,
    Kurinji,
    bindings::Bindings,
    serde::BindingsSerdeHelper,
};
use bevy::prelude::*;
use std::{
    collections::HashMap,
    fs,
};
impl Kurinji
{
    // publics
    pub fn get_bindings_from_json_file(path: &str) -> Bindings
    {
        let json = fs::read_to_string(path).expect("Error! could not open \
                                                    bindings file");
        Kurinji::get_bindings_from_json(&json)
    }

    pub fn get_bindings_from_ron_file(path: &str) -> Bindings
    {
        let ron = fs::read_to_string(path).expect("Error! could not open \
                                                   bindings file");
        Kurinji::get_bindings_from_ron(&ron)
    }

    pub fn get_bindings_from_json(json: &str) -> Bindings
    {
        serde_json::from_str(json).expect("Failed to deserialise bindings json")
    }

    pub fn get_bindings_from_ron(ron: &str) -> Bindings
    {
        ron::de::from_str(ron).expect("Failed to deserialise bindings ron")
    }

    // crate
    pub(crate) fn get_pad_axis_from_bevy_gamepad_axis_type(
        axis_type: bevy::input::gamepad::GamepadAxisType,
        str: f32)
        -> Option<GamepadAxis>
    {
        match axis_type
        {
            bevy::input::gamepad::GamepadAxisType::LeftStickX =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::LeftStickXPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::LeftStickXNegative);
                }
                None
            }
            bevy::input::gamepad::GamepadAxisType::LeftStickY =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::LeftStickYPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::LeftStickYNegative);
                }
                None
            }
            bevy::input::gamepad::GamepadAxisType::LeftZ =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::LeftZPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::LeftZNegative);
                }
                None
            }

            bevy::input::gamepad::GamepadAxisType::RightStickX =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::RightStickXPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::RightStickXNegative);
                }
                None
            }

            bevy::input::gamepad::GamepadAxisType::RightStickY =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::RightStickYPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::RightStickYNegative);
                }
                None
            }
            bevy::input::gamepad::GamepadAxisType::RightZ =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::RightZPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::RightZNegative);
                }
                None
            }
            bevy::input::gamepad::GamepadAxisType::DPadX =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::DPadXPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::DPadXNegative);
                }
                None
            }
            bevy::input::gamepad::GamepadAxisType::DPadY =>
            {
                if str > 0.0
                {
                    return Some(GamepadAxis::DPadYPositive);
                }
                if str < 0.0
                {
                    return Some(GamepadAxis::DPadYNegative);
                }
                None
            }
        }
    }
}
impl BindingsSerdeHelper
{
    // utils
    pub fn get_gamepad_button_hash_map_from_json_friendly_map(
        json_map: HashMap<usize, HashMap<GamepadButtonType, String>>)
        -> HashMap<(usize, GamepadButtonType), String>
    {
        let mut result: HashMap<(usize, GamepadButtonType), String> =
            HashMap::new();
        for (player, h_map) in json_map
        {
            for (btn_type, action) in h_map
            {
                result.entry((player, btn_type)).or_insert(action);
            }
        }
        result
    }

    pub fn get_gamepad_axis_hash_map_from_json_friendly_map(
        json_map: HashMap<usize, HashMap<GamepadAxis, String>>)
        -> HashMap<(usize, GamepadAxis), String>
    {
        let mut result: HashMap<(usize, GamepadAxis), String> = HashMap::new();
        for (pad_handle, h_map) in json_map
        {
            for (g_axis, action) in h_map
            {
                result.entry((pad_handle, g_axis)).or_insert(action);
            }
        }
        result
    }

    pub fn get_json_friendly_gamepad_button_hash_map(
        binding: HashMap<(usize, GamepadButtonType), String>)
        -> HashMap<usize, HashMap<GamepadButtonType, String>>
    {
        let mut result: HashMap<usize, HashMap<GamepadButtonType, String>> =
            HashMap::new();
        for (k, v) in binding
        {
            let id: usize = k.0;
            let button = k.1;
            let action = v;
            result.entry(id).or_insert_with(HashMap::new);
            result.get_mut(&id).unwrap().insert(button, action);
        }
        result
    }

    pub fn get_json_friendly_gamepad_axis_hash_map(
        binding: HashMap<(usize, GamepadAxis), String>)
        -> HashMap<usize, HashMap<GamepadAxis, String>>
    {
        let mut result: HashMap<usize, HashMap<GamepadAxis, String>> =
            HashMap::new();
        for (k, action) in binding
        {
            let id = k.0;
            let axis_helper = k.1;
            result.entry(id).or_insert_with(HashMap::new);
            result.get_mut(&id).unwrap().insert(axis_helper, action);
        }
        result
    }
}
pub(crate) fn normalised_within_range(min: f32, max: f32, value: f32) -> f32
{
    // src: https://stats.stackexchange.com/questions/70801/how-to-normalize-data-to-0-1-range
    // formula: newvalue= (max'-min')/(max-min)*(value-max)+max'
    // new arbitrary range min' to max'
    let r = 1. / (max - min) * (value - max) + 1.0;
    clamp(0., 1., r)
}
pub(crate) fn clamp(min: f32, max: f32, value: f32) -> f32
{
    if value < min
    {
        return min;
    }
    else if value > max
    {
        return max;
    }
    value
}
pub(crate) fn clamp_vec2(min: Vec2, max: Vec2, value: Vec2) -> Vec2
{
    Vec2::new(clamp(min.x, max.x, value.x), clamp(min.y, max.y, value.y))
}

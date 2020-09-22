use bevy::prelude::{GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType, KeyCode, MouseButton};
use std::{collections::HashMap};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{axis::{AnalogDirection, Axis}, bindings::Bindings, eventphase::EventPhase, gamepad::GamepadAnalog, inputmap::{InputMap}};

impl  InputMap {
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
            Err(e) => Err("Failed to generate ron".to_string()),
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
            Err(e) => Err("Failed to generate json".to_string()),
        }
    }
    pub fn set_bindings_with_json(&mut self, json: &str) {
        let bindings: Bindings = InputMap::get_bindings_from_json(json);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum GamepadAxisHelper
{
    LeftStickX_Positive,
    LeftStickX_Negative,

    LeftStickY_Positive,
    LeftStickY_Negative,

    LeftZ_Positive,
    LeftZ_Negative,

    RightStickX_Positive,
    RightStickX_Negative,

    RightStickY_Positive,
    RightStickY_Negative,

    RightZ_Positive,
    RightZ_Negative,

    DPadX_Positive,
    DPadX_Negative,

    DPadY_Positive,
    DPadY_Negative,
}

impl Default for GamepadAxisHelper {
    fn default() -> Self {
        GamepadAxisHelper::LeftStickX_Positive
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct BindingsHelper
{
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
impl BindingsHelper {
    pub fn GetJSONFriendlyGamepadButtonHashMap(binding: HashMap<GamepadButton, String>) 
    -> HashMap<usize, HashMap<GamepadButtonType, String>>
    {
        let mut result: HashMap<usize, HashMap<GamepadButtonType, String>> = HashMap::new();
    
        for (k, v) in binding
        {
            let id:usize = k.0.0;
            let button = k.1;
            let action = v;

            if !result.contains_key(&id)
            {
                result.insert(id, HashMap::new());
            }

            result.get_mut(&id).unwrap().insert(button, action);
        }
        result
    }
    pub fn GetJSONFriendlyGamepadAnalogHashMap(binding: HashMap<GamepadAnalog, String>) 
    -> HashMap<usize, HashMap<GamepadAxisHelper, String>>
    {
        let mut result: HashMap<usize, HashMap<GamepadAxisHelper, String>> = HashMap::new();
    
        for (k, v) in binding
        {
            let id = k.GamepadAxis.0.0;
            let analog = BindingsHelper::GetGamepadAxisHelper(k);
            let action = v;

            if !result.contains_key(&id)
            {
                result.insert(id, HashMap::new());
            }

            result.get_mut(&id).unwrap().insert(analog, action);
        }
        result
    }

    pub fn GetGamepadAxisHelper(analog: GamepadAnalog) -> GamepadAxisHelper
    {
        let game_axis_type = analog.GamepadAxis.1;
        let direction = analog.Direction;

        match (game_axis_type, direction)
        {
            (GamepadAxisType::LeftStickX, AnalogDirection::Positve) => GamepadAxisHelper::LeftStickX_Positive,
            (GamepadAxisType::LeftStickX, AnalogDirection::Negative) => GamepadAxisHelper::LeftStickX_Negative,

            (GamepadAxisType::LeftStickY, AnalogDirection::Positve) => GamepadAxisHelper::LeftStickY_Positive,
            (GamepadAxisType::LeftStickY, AnalogDirection::Negative) => GamepadAxisHelper::LeftStickY_Negative,

            (GamepadAxisType::LeftZ, AnalogDirection::Positve) => GamepadAxisHelper::LeftZ_Positive,
            (GamepadAxisType::LeftZ, AnalogDirection::Negative) => GamepadAxisHelper::LeftZ_Negative,

            (GamepadAxisType::RightStickX, AnalogDirection::Positve) => GamepadAxisHelper::RightStickX_Positive,
            (GamepadAxisType::RightStickX, AnalogDirection::Negative) => GamepadAxisHelper::RightStickX_Negative,

            (GamepadAxisType::RightStickY, AnalogDirection::Positve) => GamepadAxisHelper::RightStickY_Positive,
            (GamepadAxisType::RightStickY, AnalogDirection::Negative) => GamepadAxisHelper::RightStickY_Negative,
            
            (GamepadAxisType::RightZ, AnalogDirection::Positve) => GamepadAxisHelper::RightZ_Positive,
            (GamepadAxisType::RightZ, AnalogDirection::Negative) => GamepadAxisHelper::RightZ_Negative,

            (GamepadAxisType::DPadX, AnalogDirection::Positve) => GamepadAxisHelper::DPadX_Positive,
            (GamepadAxisType::DPadX, AnalogDirection::Negative) => GamepadAxisHelper::DPadX_Negative,

            (GamepadAxisType::DPadY, AnalogDirection::Positve) => GamepadAxisHelper::DPadY_Positive,
            (GamepadAxisType::DPadY, AnalogDirection::Negative) => GamepadAxisHelper::DPadY_Negative,
            
            _ => Default::default()
        }
    }
}
impl Serialize for Bindings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {

        BindingsHelper{
            // gmpad_button_bindings: 
            keyboard_key_bindings: self.keyboard_key_bindings.clone(),
            mouse_button_binding: self.mouse_button_binding.clone(),
            mouse_move_binding: self.mouse_move_binding.clone(),
            action_deadzone: self.action_deadzone.clone(),
            action_phase: self.action_phase.clone(),
            gamepad_button_bindings: BindingsHelper::GetJSONFriendlyGamepadButtonHashMap(self.gamepad_button_bindings.clone()),
            gamepad_axis_bindings: BindingsHelper::GetJSONFriendlyGamepadAnalogHashMap(self.gamepad_axis_bindings.clone()),
        }.serialize(serializer)

    }
}

impl<'de> Deserialize<'de> for Bindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer)
            .map(|BindingsHelper{
            gamepad_button_bindings,
            gamepad_axis_bindings,
            keyboard_key_bindings,
            mouse_button_binding,
            mouse_move_binding,
            action_deadzone,
            action_phase,
            }| 
            Bindings
            {
                keyboard_key_bindings,
                mouse_button_binding,
                mouse_move_binding,
                action_deadzone,
                action_phase,

                gamepad_button_bindings: Default::default(),
                gamepad_axis_bindings: Default::default()
            })
    }
}


// helpers
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GamepadBindingsSerdeHelper
{   
    #[serde(default, rename = "Gamepad")]
    pub handle: usize,

    #[serde(default, rename = "ButtonBindings")]
    pub button_bindings: HashMap<
        GamepadButtonType, 
        String>,

    // #[serde(default, rename = "AnalogBindings")]
    // pub analog_bindings: HashMap<
    // GamepadAxisBindingsSerdeHelper, 
    // String>
}


// #[derive(Serialize, Deserialize)]
// #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
// pub struct GamepadAxisBindingsSerdeHelper
// {
//     #[serde(default, rename = "Kind")]
//     axis_type: GamepadAxisType,

//     #[serde(default, rename = "Direction")]
//     direction: AnalogDirection
// }
// use std::collections::HashMap;
// use bevy_app::{self};
// use bevy::prelude::{KeyCode, MouseButton};
// use serde::{Serialize, Deserialize};

// use crate::{axis::Axis, keyboard::KeyboardMap, mouse::MouseMap};

// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct ConfigData
// {
//     keyboard_key_bindings: HashMap<KeyCode, String>,
//     mouse_button_binding: HashMap<MouseButton, String>,
//     // mouse_axis_binding: HashMap<Axis, String>
// }

// impl Config{

//     // public
//     pub fn get_bindings_as_json(&self) ->  Result<String, String>
//     {
//         let data = ConfigData
//         {
//             keyboard_key_bindings: self.keyboard_map.get_bindings(),
//             mouse_button_binding: self.mouse_map.get_button_bindings()
//         };
//         let serialized = serde_json::to_string(&data);
//         match serialized {
//             Ok(s) => Ok(s),
//             Err(e) => Err("Failed to generate json".to_string()),
//         }
//     }

//     pub fn set_bindings_with_json(&self, string: String)
//     {
        
//     } 

//     // crates
//     pub(crate) fn init(&mut self, k: &mut KeyboardMap, m: &mut MouseMap)
//     {
//         self.mouse_map = m;
//         self.keyboard_map = k;
//     }  
// }
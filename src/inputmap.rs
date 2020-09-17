use bevy::prelude::{KeyCode, MouseButton};
use bevy_ecs::ResMut;
use std::collections::HashMap;
use crate::{util, axis::Axis, bindings::Bindings};

#[derive(Default)]
pub struct InputMap {
    // crates
    // actions
    pub(crate) action_strength_curve: HashMap<String, fn(f32) -> f32>,
    pub(crate) action_raw_strength: HashMap<String, f32>,
    pub(crate) action_deadzone: HashMap<String, f32>,

    // keyboard
    pub(crate) keyboard_action_binding: HashMap<KeyCode, String>,

    // mouse
    pub(crate) mouse_button_binding: HashMap<MouseButton, String>,
    pub(crate) mouse_move_binding: HashMap<Axis, String>,

    // stack
    pub(crate) stack: Vec<Bindings>,
}
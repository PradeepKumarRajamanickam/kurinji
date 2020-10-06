use bevy::prelude::{Gamepad, GamepadButton, KeyCode, MouseButton};

use crate::{axis::Axis, bindings::Bindings, EventPhase, gamepad::GamepadAnalog};
use std::collections::{HashMap, HashSet};

/// Resource to access all Input Map APIs
#[derive(Default)]
pub struct InputMap {
    // crates
    // actions
    pub(crate) action_strength_curve: HashMap<String, fn(f32) -> f32>,
    pub(crate) action_raw_strength: HashMap<String, f32>,
    pub(crate) action_prev_strength: HashMap<String, f32>, // strength value from prev frame
    pub(crate) action_deadzone: HashMap<String, f32>,
    pub(crate) action_phase: HashMap<String, EventPhase>,

    // keyboard
    pub(crate) keyboard_action_binding: HashMap<KeyCode, String>,

    // mouse
    pub(crate) mouse_button_binding: HashMap<MouseButton, String>,
    pub(crate) mouse_move_binding: HashMap<Axis, String>,

    // joystick
    pub(crate) joystick_connected_handle: HashSet<Gamepad>,
    pub(crate) joystick_button_binding: HashMap<GamepadButton, String>,
    pub(crate) joystick_axis_binding: HashMap<GamepadAnalog, String>,

    // stack
    pub(crate) stack: Vec<Bindings>,
}

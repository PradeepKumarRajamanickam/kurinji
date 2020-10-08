use crate::{axis::MouseAxis, bindings::Bindings, EventPhase, GamepadAxis};

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

impl InputMap {
    // constants
    /// Max number of players that can be connected using gamepads
    pub const MAX_PLAYER_HANDLES: usize = 8;
}

/// Resource to access all Input Map APIs
#[derive(Default, Clone)]
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
    pub(crate) mouse_move_binding: HashMap<MouseAxis, String>,

    // joystick
    pub(crate) player_handles_in_use: HashSet<usize>,
    pub(crate) joystick_to_player_map: HashMap<Gamepad, usize>,
    pub(crate) player_to_joystick_map: HashMap<usize, Gamepad>,

    pub(crate) joystick_button_binding: HashMap<(usize, GamepadButtonType), String>,
    pub(crate) joystick_axis_binding: HashMap<(usize, GamepadAxis), String>,

    // stack
    pub(crate) stack: Vec<Bindings>,
}

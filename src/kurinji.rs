use crate::{axis::MouseAxis, bindings::Bindings, EventPhase, GamepadAxis, Actionable};

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

impl<T: Actionable> Kurinji<T> {
    // constants
    /// Max number of players that can be connected using gamepads
    pub const MAX_PLAYER_HANDLES: usize = 8;
}

/// Resource to access all Kurinji APIs
#[derive(Default)]
pub struct Kurinji<T: Actionable> {
    // crates
    // actions
    pub(crate) action_strength_curve: HashMap<T, fn(f32) -> f32>,
    pub(crate) action_raw_strength: HashMap<T, f32>,
    pub(crate) action_prev_strength: HashMap<T, f32>, // strength value from prev frame
    pub(crate) action_deadzone: HashMap<T, f32>,
    pub(crate) action_phase: HashMap<T, EventPhase>,

    // keyboard
    pub(crate) keyboard_action_binding: HashMap<KeyCode, T>,

    // mouse
    pub(crate) mouse_button_binding: HashMap<MouseButton, T>,
    pub(crate) mouse_move_binding: HashMap<MouseAxis, T>,

    // joystick
    pub(crate) player_handles_in_use: HashSet<usize>,
    pub(crate) joystick_to_player_map: HashMap<Gamepad, usize>,
    pub(crate) player_to_joystick_map: HashMap<usize, Gamepad>,

    pub(crate) joystick_button_binding: HashMap<(usize, GamepadButtonType), T>,
    pub(crate) joystick_axis_binding: HashMap<(usize, GamepadAxis), T>,

    // stack
    pub(crate) stack: Vec<Bindings<T>>,
}

use crate::{Axis, InputMap, util::clamp_vec2};
use bevy::{math::Vec2, prelude::MouseButton};
use bevy_app::{EventReader, Events};
use bevy_ecs::{Local, Res, ResMut};
use bevy_input::{mouse::MouseMotion, Input};

#[derive(Default)]
pub struct MouseMoveState {
    reader: EventReader<MouseMotion>,
}

impl InputMap {
    // publics
    pub fn bind_mouse_button_pressed(&mut self, code: MouseButton, action: &str) -> &mut InputMap {
        self.mouse_button_binding.insert(code, action.to_string());
        self
    }

    pub fn unbind_mouse_button_pressed(&mut self, button: MouseButton) -> &mut InputMap {
        self.mouse_button_binding.remove(&button);
        self
    }

    pub fn bind_mouse_motion(&mut self, axis: Axis, action: &str) -> &mut InputMap {
        self.mouse_move_binding.insert(axis, action.to_string());
        self
    }

    pub fn unbind_mouse_motion(&mut self, axis: Axis) -> &mut InputMap {
        self.mouse_move_binding.remove(&axis);
        self
    }

    // crates
    // systems
    pub(crate) fn mouse_button_press_input_system(
        mut input_map: ResMut<InputMap>,
        mouse_button_input: Res<Input<MouseButton>>,
    ) {
        let button_bindings_iter = input_map.mouse_button_binding.clone();
        for (button, action) in button_bindings_iter.iter() {
            if mouse_button_input.pressed(*button) {
                input_map.set_raw_action_strength(action, 1.0);
            }
        }
    }

    pub(crate) fn mouse_move_event_system(
        mut input_map: ResMut<InputMap>,
        mut state: Local<MouseMoveState>,
        move_events: Res<Events<MouseMotion>>,
    ) {
        if let Some(value) = state.reader.latest(&move_events) {
            // normalises strength (with 10 px as max strength)
            // i.e. -10 px = -1.0 and 10 px = 1.0
            let min = -1. * Vec2::one();
            let max = Vec2::one();
            let normalised_vec = clamp_vec2(min, max, value.delta * 0.1);

            let x = normalised_vec.x();
            let y = normalised_vec.y();

            // horizontal
            if x > 0.0 {
                if let Some(action) = input_map.mouse_move_binding.get(&Axis::XPositive) {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, x);
                }
            }

            if x < 0.0 {
                if let Some(action) = input_map.mouse_move_binding.get(&Axis::XNegative) {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, x.abs());
                }
            }

            // vertical
            if y > 0.0 {
                if let Some(action) = input_map.mouse_move_binding.get(&Axis::YPositive) {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, y);
                }
            }

            if y < 0.0 {
                if let Some(action) = input_map.mouse_move_binding.get(&Axis::YNegative) {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, y.abs());
                }
            }
        }
    }
}

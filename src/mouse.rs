use crate::{MouseAxis, Kurinji, util::clamp_vec2};
use bevy::{math::Vec2, prelude::MouseButton};
use bevy::app::EventReader;
use bevy::ecs::system::{Res, ResMut};
use bevy::input::{mouse::MouseMotion, Input};
impl Kurinji {
    // publics
    pub fn bind_mouse_button_pressed(
        &mut self,
        code: MouseButton,
        action: &str,
    ) -> &mut Kurinji {
        self.mouse_button_binding.insert(code, action.to_string());
        self
    }

    pub fn unbind_mouse_button_pressed(
        &mut self,
        button: MouseButton,
    ) -> &mut Kurinji {
        self.mouse_button_binding.remove(&button);
        self
    }

    pub fn bind_mouse_motion(
        &mut self,
        axis: MouseAxis,
        action: &str,
    ) -> &mut Kurinji {
        self.mouse_move_binding.insert(axis, action.to_string());
        self
    }

    pub fn unbind_mouse_motion(&mut self, axis: MouseAxis) -> &mut Kurinji {
        self.mouse_move_binding.remove(&axis);
        self
    }

    // crates
    // systems
    pub(crate) fn mouse_button_press_input_system(
        mut input_map: ResMut<Kurinji>,
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
        mut input_map: ResMut<Kurinji>,
        mut reader: EventReader<MouseMotion>,
    ) {
        for event in reader.iter() {
            // normalises strength (with 10 px as max strength)
            // i.e. -10 px = -1.0 and 10 px = 1.0
            let min = -1. * Vec2::ONE;
            let max = Vec2::ONE;
            let normalised_vec = clamp_vec2(min, max, event.delta * 0.1);
            let x = normalised_vec.x;
            let y = normalised_vec.y;
            // horizontal
            if x > 0.0 {
                if let Some(action) =
                    input_map.mouse_move_binding.get(&MouseAxis::XPositive)
                {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, x);
                }
            }
            if x < 0.0 {
                if let Some(action) =
                    input_map.mouse_move_binding.get(&MouseAxis::XNegative)
                {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, x.abs());
                }
            }
            // vertical
            if y > 0.0 {
                if let Some(action) =
                    input_map.mouse_move_binding.get(&MouseAxis::YPositive)
                {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, y);
                }
            }
            if y < 0.0 {
                if let Some(action) =
                    input_map.mouse_move_binding.get(&MouseAxis::YNegative)
                {
                    let _action = action.clone();
                    input_map.set_raw_action_strength(&_action, y.abs());
                }
            }
        }
    }
}

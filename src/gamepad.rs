use crate::{axis::Axis, inputmap::InputMap, util::clamp_vec2};
use bevy::{
    math::Vec2, prelude::Gamepad, prelude::GamepadButton, prelude::GamepadButtonType,
    prelude::GamepadEvent,
};
use bevy_app::{EventReader, Events};
use bevy_ecs::{Local, Res, ResMut};
use bevy_input::Input;

impl InputMap {
    // publics
    pub fn bind_gamepad_button_pressed(
        &mut self,
        joy_button: GamepadButtonType,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_button_pressed_with_gamepad_number(0, joy_button, action)
    }
    pub fn bind_gamepad_button_pressed_with_gamepad_number(
        &mut self,
        pad_number: usize,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_button_pressed_with_gamepad_button(
            GamepadButton(Gamepad(pad_number), pad_button),
            action,
        )
    }
    pub fn bind_gamepad_button_pressed_with_gamepad_button(
        &mut self,
        button: GamepadButton,
        action: &str,
    ) -> &mut InputMap {
        self.joystick_button_binding
            .insert(button, action.to_string());
        self
    }

    pub fn unbind_gamepad_button_pressed(&mut self, 
    joy_button: GamepadButtonType) -> &mut InputMap {
        self.unbind_gamepad_button_pressed_with_gamepad_number(0, joy_button)
    }
    pub fn unbind_gamepad_button_pressed_with_gamepad_number(
        &mut self,
        pad_number: usize,
        pad_button: GamepadButtonType,
    ) -> &mut InputMap {
        self.unbind_gamepad_button_pressed_with_gamepad_button(GamepadButton(
            Gamepad(pad_number),
            pad_button,
        ))
    }
    pub fn unbind_gamepad_button_pressed_with_gamepad_button(
        &mut self,
        button: GamepadButton,
    ) -> &mut InputMap {
        self.joystick_button_binding.remove(&button);
        self
    }

    // crates
    // systems
    pub(crate) fn gamepad_button_press_input_system(
        mut input_map: ResMut<InputMap>,
        joystick_button_input: Res<Input<GamepadButton>>,
    ) {
        let button_bindings_iter = input_map.joystick_button_binding.clone();
        for (button, action) in button_bindings_iter.iter() {
            if joystick_button_input.pressed(*button) {
                input_map.set_raw_action_strength(action, 1.0);
            }
        }
    }

    
}

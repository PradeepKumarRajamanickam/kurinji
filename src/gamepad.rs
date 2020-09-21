use crate::{axis::AnalogDirection, axis::Axis, inputmap::InputMap, util::clamp_vec2};
use bevy::{
    math::Vec2, prelude::Gamepad, prelude::GamepadAxis, prelude::GamepadAxisType,
    prelude::GamepadButton, prelude::GamepadButtonType, prelude::GamepadEvent,
};
use bevy_app::{EventReader, Events};
use bevy_ecs::{Local, Res, ResMut};
use bevy_input::Input;

#[derive(Default)]
pub struct GamepadState {
    reader: EventReader<GamepadEvent>,
}

impl InputMap {
    // publics
    // buttons
    pub fn bind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_button_pressed_with_gamepad_handle(0, pad_button, action)
    }
    pub fn bind_gamepad_button_pressed_with_gamepad_handle(
        &mut self,
        pad_handle: usize,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_button_pressed_with_gamepad_button(
            GamepadButton(Gamepad(pad_handle), pad_button),
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
    pub fn unbind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
    ) -> &mut InputMap {
        self.unbind_gamepad_button_pressed_with_gamepad_handle(0, pad_button)
    }
    pub fn unbind_gamepad_button_pressed_with_gamepad_handle(
        &mut self,
        pad_handle: usize,
        pad_button: GamepadButtonType,
    ) -> &mut InputMap {
        self.unbind_gamepad_button_pressed_with_gamepad_button(GamepadButton(
            Gamepad(pad_handle),
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

    // axis
    pub fn bind_gamepad_axis(
        &mut self,
        pad_axis: GamepadAxis,
        analog_direction: AnalogDirection,
        action: &str,
    ) -> &mut InputMap {
        self.joystick_axis_binding
            .insert((pad_axis, analog_direction), action.to_string());
        self
    }

    pub fn unbind_gamepad_axis(
        &mut self,
        pad_axis: GamepadAxis,
        analog_direction: AnalogDirection,
    ) -> &mut InputMap {
        self.joystick_axis_binding
            .remove(&(pad_axis, analog_direction));
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
    pub(crate) fn gamepad_connection_event_system(
        mut input_map: ResMut<InputMap>,
        gamepad_event: Res<Events<GamepadEvent>>,
        mut state: Local<GamepadState>,
    ) {
        if let Some(value) = state.reader.latest(&gamepad_event) {
            let pad_handle = value.0;
            match value.1 {
                Connected => {
                    println!("InputMap: Joystick Connected {:?}", pad_handle);
                    input_map.joystick_connected_handle.insert(pad_handle);
                }
                Disconnected => {
                    println!("InputMap: Joystick Disconnected {:?}", pad_handle);
                    input_map.joystick_connected_handle.remove(&pad_handle);
                }
            }
        }
    }

    pub(crate) fn gamepad_axis_system(
        mut input_map: ResMut<InputMap>,
        pad_axis: Res<bevy_input::Axis<GamepadAxis>>,
    ) {
        let connected_pads = input_map.joystick_connected_handle.clone();
        for pad_handle in connected_pads.iter(){
            for (k, v) in input_map.joystick_axis_binding.clone().iter() {
                let g_axis = k.0;
                let a_dir = k.1;

                let signed_str = pad_axis.get(&g_axis).unwrap_or(0.);

                if signed_str > 0. && a_dir == AnalogDirection::Positve {
                    input_map.set_raw_action_strength(&v.to_string(), signed_str);
                }
                else if signed_str < 0. && a_dir == AnalogDirection::Negative {
                    input_map.set_raw_action_strength(&v.to_string(), signed_str.abs());
                }
            }
        }
    }
}

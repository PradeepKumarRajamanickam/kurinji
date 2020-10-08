use crate::{GamepadAxis, InputMap};

use bevy::{
    prelude::Gamepad, prelude::GamepadButton,
    prelude::GamepadButtonType, prelude::GamepadEvent,
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
        self.bind_gamepad_button_pressed_with_player(0, pad_button, action)
    }
    pub fn bind_gamepad_button_pressed_with_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut InputMap {
        *self.joystick_button_binding
        .entry((player, pad_button))
        .or_default() = action.to_string();
        self
    }
    pub fn unbind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
    ) -> &mut InputMap {
        self.unbind_gamepad_button_pressed_with_player(0, pad_button)
    }
    pub fn unbind_gamepad_button_pressed_with_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
    ) -> &mut InputMap {
        self.joystick_button_binding.remove(&(player, pad_button));
        self
    }

    // axis
    pub fn bind_gamepad_axis(
        &mut self,
        axis: GamepadAxis,
        action: &str,
    ) -> &mut InputMap {
        self.bind_gamepad_axis_for_player(0, axis, action)
    }

    pub fn bind_gamepad_axis_for_player(
        &mut self,
        player: usize,
        axis: GamepadAxis,
        action: &str,
    ) -> &mut InputMap {
        *self.joystick_axis_binding
        .entry((player, axis))
        .or_default() = action.to_string();
        self
    }

    pub fn unbind_gamepad_axis(
        &mut self,
        pad_axis: GamepadAxis,
    ) -> &mut InputMap {
        self.unbind_gamepad_axis_for_player(0, pad_axis);
        self
    }

    pub fn unbind_gamepad_axis_for_player(
        &mut self,
        player: usize,
        axis: GamepadAxis,
    ) -> &mut InputMap {
        self.joystick_axis_binding.remove(&(player, axis));
        self
    }

    // crates
    pub(crate) fn get_available_player_handle(self) -> Option<usize>
    {
        let max_player_handles: usize = 8;
        for i in 0..(max_player_handles - 1)
        {
            if !self.player_handles_in_use.contains(&i) {
               return Some(i);
            }
        }
        None
    }
    pub(crate) fn get_player_handle_for_gamepad(self, pad: Gamepad) -> Option<usize>
    {
        return match self.joystick_to_player_map.get(&pad) {
            Some(a) => Some(*a),
            _ => None
        };
    }
    pub(crate) fn get_gamepad_from_player_handle(self, player: usize) -> Option<Gamepad>
    {
        return match self.player_to_joystick_map.get(&player) {
            Some(a) => Some(*a),
            _ => None
        };
    }
    // systems
    pub(crate) fn gamepad_button_press_input_system(
        mut input_map: ResMut<InputMap>,
        joystick_button_input: Res<Input<GamepadButton>>,
    ) {
        let button_bindings_iter = input_map.joystick_button_binding.clone();
        for (player_button_bind, action) in button_bindings_iter.iter() {
            if joystick_button_input.pressed(GamepadButton(Gamepad(player_button_bind.0), player_button_bind.1)) {
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
                bevy::prelude::GamepadEventType::Connected => {

                    let res_player_handle = input_map.clone().get_available_player_handle();
                    match res_player_handle {

                        Some(player_handle) => 
                        {
                            println!("InputMap: Gamepad Connected {:?} to player {}", pad_handle, player_handle);
                            input_map.player_handles_in_use.insert(player_handle);
                            input_map.joystick_to_player_map.insert(pad_handle,player_handle);
                            input_map.player_to_joystick_map.insert(player_handle,pad_handle);
                        }
                        None => { println!("InputMap: Housefull. No space for more gamepads"); }
                    }
                        
                }
                bevy::prelude::GamepadEventType::Disconnected => {

                    let opt_player_handle = input_map.clone().get_player_handle_for_gamepad(pad_handle);
                    if let Some(player_handle) = opt_player_handle
                    {
                        println!("InputMap: Gamepad Disconnected {:?} for player {}", pad_handle, player_handle);
                        input_map.player_handles_in_use.remove(&player_handle);
                        input_map.joystick_to_player_map.remove(&pad_handle);
                        input_map.player_to_joystick_map.remove(&player_handle);
                    }
                }
            }
        }
    }

    pub(crate) fn gamepad_axis_system(
        mut input_map: ResMut<InputMap>,
        pad_axis: Res<bevy_input::Axis<bevy_input::gamepad::GamepadAxis>>,
    ) {
        for (k, v) in input_map.clone().joystick_axis_binding.iter() {
            let player = k.0;
            let axis = k.1.clone();
            let is_positive = InputMap::is_gamepad_axis_positive(axis.clone());

            if let Some(bevy_gamepad) = input_map.clone().get_gamepad_from_player_handle(player) {
                let bevy_axis_type = InputMap::get_bevy_gamepad_axis_type_from_pad_axis(axis);
                let bevy_axis = bevy_input::gamepad::GamepadAxis(bevy_gamepad, bevy_axis_type);

                let signed_str = pad_axis.get(&bevy_axis).unwrap_or(0.);

                if signed_str > 0. && is_positive || signed_str < 0. && !is_positive {
                    input_map.set_raw_action_strength(&v.to_string(), signed_str.abs());
                }
            }
        }
    }
}

use crate::{GamepadAxis, Kurinji};

use bevy::prelude::*;
use bevy::app::{EventReader, Events};
use bevy::ecs::{Local, Res, ResMut};
use bevy::input::Input;

#[derive(Default)]
pub struct GamepadState {
    reader: EventReader<GamepadEvent>,
}

impl Kurinji {
    // publics
    // buttons
    pub fn bind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut Kurinji {
        self.bind_gamepad_button_pressed_for_player(0, pad_button, action)
    }
    pub fn bind_gamepad_button_pressed_for_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
        action: &str,
    ) -> &mut Kurinji {
        *self
            .joystick_button_binding
            .entry((player, pad_button))
            .or_default() = action.to_string();
        self
    }
    pub fn unbind_gamepad_button_pressed(
        &mut self,
        pad_button: GamepadButtonType,
    ) -> &mut Kurinji {
        self.unbind_gamepad_button_pressed_for_player(0, pad_button)
    }
    pub fn unbind_gamepad_button_pressed_for_player(
        &mut self,
        player: usize,
        pad_button: GamepadButtonType,
    ) -> &mut Kurinji {
        self.joystick_button_binding.remove(&(player, pad_button));
        self
    }

    // axis
    pub fn bind_gamepad_axis(&mut self, axis: GamepadAxis, action: &str) -> &mut Kurinji {
        self.bind_gamepad_axis_for_player(0, axis, action)
    }

    pub fn bind_gamepad_axis_for_player(
        &mut self,
        player: usize,
        axis: GamepadAxis,
        action: &str,
    ) -> &mut Kurinji {
        *self
            .joystick_axis_binding
            .entry((player, axis))
            .or_default() = action.to_string();
        self
    }

    pub fn unbind_gamepad_axis(&mut self, pad_axis: GamepadAxis) -> &mut Kurinji {
        self.unbind_gamepad_axis_for_player(0, pad_axis);
        self
    }

    pub fn unbind_gamepad_axis_for_player(
        &mut self,
        player: usize,
        axis: GamepadAxis,
    ) -> &mut Kurinji {
        self.joystick_axis_binding.remove(&(player, axis));
        self
    }

    // crates
    pub(crate) fn get_available_player_handle(self) -> Option<usize> {
        for i in 0..(Kurinji::MAX_PLAYER_HANDLES - 1) {
            if !self.player_handles_in_use.contains(&i) {
                return Some(i);
            }
        }
        None
    }
    pub(crate) fn get_player_handle_for_gamepad(self, pad: Gamepad) -> Option<usize> {
        return match self.joystick_to_player_map.get(&pad) {
            Some(a) => Some(*a),
            _ => None,
        };
    }
    pub(crate) fn get_gamepad_from_player_handle(self, player: usize) -> Option<Gamepad> {
        return match self.player_to_joystick_map.get(&player) {
            Some(a) => Some(*a),
            _ => None,
        };
    }
    // systems
    pub(crate) fn gamepad_button_press_input_system(
        mut input_map: ResMut<Kurinji>,
        joystick_button_input: Res<Input<GamepadButton>>,
    ) {
        let button_bindings_iter = input_map.joystick_button_binding.clone();
        for (player_button_bind, action) in button_bindings_iter.iter() {
            if joystick_button_input.pressed(GamepadButton(
                Gamepad(player_button_bind.0),
                player_button_bind.1,
            )) {
                input_map.set_raw_action_strength(action, 1.0);
            }
        }
    }
    pub(crate) fn gamepad_event_system(
        mut input_map: ResMut<Kurinji>,
        gamepad_event: Res<Events<GamepadEvent>>,
        mut state: Local<GamepadState>,
    ) {
        if let Some(value) = state.reader.latest(&gamepad_event) {
            let pad_handle = value.0;
            let pad_event_type = value.clone().1;
            match pad_event_type {
                bevy::prelude::GamepadEventType::AxisChanged(axis_type, axis_str) => {
                    if let Some(axis_type) =
                        Kurinji::get_pad_axis_from_bevy_gamepad_axis_type(axis_type, axis_str)
                    {
                        if let Some(action) = input_map
                            .clone()
                            .joystick_axis_binding
                            .get(&(pad_handle.0, axis_type))
                        {
                            input_map
                                .joystick_last_action_data
                                .insert(action.to_string(), axis_str.abs());
                        }
                    }
                }
                bevy::prelude::GamepadEventType::Connected => {
                    let res_player_handle = input_map.clone().get_available_player_handle();
                    match res_player_handle {
                        Some(player_handle) => {
                            println!(
                                "InputMap: Gamepad Connected {:?} to player {}",
                                pad_handle, player_handle
                            );
                            input_map.player_handles_in_use.insert(player_handle);
                            input_map
                                .joystick_to_player_map
                                .insert(pad_handle, player_handle);
                            input_map
                                .player_to_joystick_map
                                .insert(player_handle, pad_handle);
                        }
                        None => {
                            println!("InputMap: Housefull. No space for more gamepads");
                        }
                    }
                }
                bevy::prelude::GamepadEventType::Disconnected => {
                    let opt_player_handle =
                        input_map.clone().get_player_handle_for_gamepad(pad_handle);
                    if let Some(player_handle) = opt_player_handle {
                        println!(
                            "InputMap: Gamepad Disconnected {:?} for player {}",
                            pad_handle, player_handle
                        );
                        input_map.player_handles_in_use.remove(&player_handle);
                        input_map.joystick_to_player_map.remove(&pad_handle);
                        input_map.player_to_joystick_map.remove(&player_handle);
                    }
                }
                _ => (),
            }
        }

        // converting events into continuous input
        for (a, s) in input_map
        .joystick_last_action_data
        .clone() {
            input_map.set_raw_action_strength(
                &a.to_string(),
                s.abs());
        }
    }
}

use bevy_ecs::ResMut;
use std::collections::HashMap;
use crate::util;

#[derive(Default)]
pub struct InputMap {
    // crate
    // action data
    action_raw_strength: HashMap<String, f32>,
    action_deadzone: HashMap<String, f32>,
}

impl InputMap {
    // publics
    pub fn get_action_strength(&self, action: String) -> f32 {
        match self.action_raw_strength.get(&action) {
            Some(raw_strength) => *raw_strength,
            // Some(raw_strength) => match self.action_deadzone.get(&action) {
            //     Some(d) => InputMap::get_strength_after_applying_deadzone(*d, *raw_strength),
            //     None => *raw_strength,
            // },
            None => 0.,
        }
    }

    pub fn is_action_in_progress(&self, action: String) -> bool {
        self.get_action_strength(action) > 0.0
    }

    // Note* meaningful only for analog inputs like mouse move, joystick...etc
    pub fn set_dead_zone(&mut self, action: String, value: f32) {
        self.action_deadzone.insert(action, value);
    }

    // crates
    pub(crate) fn set_raw_action_strength(&mut self, action: String, strength: f32) {
        self.action_raw_strength.insert(action, strength);
    }

    pub(crate) fn reset_all_raw_strength(&mut self) {
        self.action_raw_strength.clear();
    }

    // systems
    pub(crate) fn action_reset_system(mut input_map: ResMut<InputMap>) {
        input_map.reset_all_raw_strength();
    }

    // private
    fn get_strength_after_applying_deadzone(deadzone: f32, raw_strength: f32) -> f32 {
        util::normalised_within_range(deadzone, 1.0, raw_strength)
    }
}

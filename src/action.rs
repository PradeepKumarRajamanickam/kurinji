use crate::{EventPhase, Kurinji, util, Actionable};

use bevy::ecs::ResMut;

impl<T: Actionable> Kurinji<T> {
    // publics
    /// Provides strength of action in range 0.0 - 1.0. Useful
    /// when working actions that mapped to analog input eg. joystick
    pub fn get_action_strength(&self, action: T) -> f32 {
        match self.action_raw_strength.get(&action) {
            Some(raw_strength) => match self.action_deadzone.get(&action) {
                Some(d) => {
                    let strength =
                        Kurinji::<T>::get_strength_after_applying_deadzone(*d, *raw_strength);
                    if let Some(curve_func) = self.action_strength_curve.get(&action) {
                        return curve_func(strength);
                    }
                    strength
                }
                None => *raw_strength,
            },
            None => 0.,
        }
    }

    /// Is this action happening.
    /// Note* this depends on action event phase
    pub fn is_action_active(&self, action: T) -> bool {
        match self.get_event_phase(action) {
            EventPhase::OnBegin => self.did_action_just_began(action),
            EventPhase::OnProgress => self.is_action_in_progress(action),
            EventPhase::OnEnded => self.did_action_just_end(action),
        }
    }

    /// Set a dead zone threshold i.e. strenght will be 0.0 until
    /// threshold is met. The strength range 0.0 - 1.0 is now mapped to
    /// min_threshold - 1.0
    ///
    /// Note* meaningful only for analog inputs like joystick,
    /// mouse move delta...etc
    pub fn set_dead_zone(&mut self, action: T, value: f32) -> &mut Kurinji<T> {
        self.action_deadzone.insert(action, value);
        self
    }

    /// Set a custom curve function that will be applied to
    /// actions strength.
    pub fn set_strength_curve_function(
        &mut self,
        action: T,
        function: fn(f32) -> f32,
    ) -> &mut Kurinji<T> {
        self.action_strength_curve
            .insert(action, function);
        self
    }

    // crates
    pub(crate) fn get_prev_strength(&self, action: T) -> f32 {
        if let Some(v) = self.action_prev_strength.get(&action) {
            return *v;
        }

        0.
    }
    pub(crate) fn set_raw_action_strength(&mut self, action: T, strength: f32) {
        self.action_raw_strength
            .insert(action, strength);
    }

    pub(crate) fn reset_all_raw_strength(&mut self) {
        self.action_raw_strength.clear();
    }

    // systems
    pub(crate) fn action_reset_system(mut input_map: ResMut<Kurinji<T>>) {
        // cache prev frame
        input_map.action_prev_strength.clear();
        for k in input_map.action_raw_strength.clone().keys() {
            let strength = input_map.get_action_strength(*k);
            input_map.action_prev_strength.insert(k.clone(), strength);
        }

        input_map.reset_all_raw_strength();
    }

    // private
    fn get_strength_after_applying_deadzone(deadzone: f32, raw_strength: f32) -> f32 {
        util::normalised_within_range(deadzone, 1.0, raw_strength)
    }
}

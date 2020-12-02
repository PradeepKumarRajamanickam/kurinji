// publics
pub use self::
{
    kurinji::Kurinji,

    axis::MouseAxis,
    axis::GamepadAxis,
    
    bindings::Bindings,
    
    event_phase::EventPhase,
    
    action_event::OnActionBegin,
    action_event::OnActionActive,
    action_event::OnActionProgress,
    action_event::OnActionEnd,
};

use ::serde::{Serialize, de::DeserializeOwned};

// crates
mod axis;
mod util;
mod stack;
mod bindings;
mod kurinji;
mod event_phase;
mod action_event;

mod action;
mod gamepad;
mod keyboard;
mod mouse;
mod serde;

use bevy::app::prelude::*;
use bevy::ecs::IntoQuerySystem;


// Using this like a trait alias
pub trait Actionable: 'static + Send + Sync + Default + Copy + Clone + Eq + PartialEq + std::hash::Hash + Serialize + DeserializeOwned{}

/// Adds input mapping (via code or json/ron) to an App
#[derive(Default)]
pub struct KurinjiPlugin<T: Actionable>{
    phantom: std::marker::PhantomData<T>
}

impl<T:Actionable> Plugin for KurinjiPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app
            // input map
            .init_resource::<Kurinji<T>>()
            // events
            .add_event::<OnActionActive<T>>()
            .add_event::<OnActionBegin<T>>()
            .add_event::<OnActionProgress<T>>()
            .add_event::<OnActionEnd<T>>()
            .add_system_to_stage(stage::EVENT, Kurinji::<T>::action_event_producer.system())
            // reset
            .add_system_to_stage(stage::PRE_UPDATE, Kurinji::<T>::action_reset_system.system())
            // joystick
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::<T>::gamepad_connection_event_system.system(),
            )
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::<T>::gamepad_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, Kurinji::<T>::gamepad_axis_system.system())
            // keyboard
            .add_system_to_stage(stage::UPDATE, Kurinji::<T>::kb_key_press_input_system.system())
            // mouse
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::<T>::mouse_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, Kurinji::<T>::mouse_move_event_system.system());
    }
}

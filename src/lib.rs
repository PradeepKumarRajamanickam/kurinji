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

/// Adds input mapping (via code or json/ron) to an App
#[derive(Default)]
pub struct KurinjiPlugin;

impl Plugin for KurinjiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // input map
            .init_resource::<Kurinji>()
            // events
            .add_event::<OnActionActive>()
            .add_event::<OnActionBegin>()
            .add_event::<OnActionProgress>()
            .add_event::<OnActionEnd>()
            .add_system_to_stage(stage::EVENT, Kurinji::action_event_producer.system())
            // reset
            .add_system_to_stage(stage::PRE_UPDATE, Kurinji::action_reset_system.system())
            // joystick
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::gamepad_connection_event_system.system(),
            )
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::gamepad_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, Kurinji::gamepad_axis_system.system())
            // keyboard
            .add_system_to_stage(stage::UPDATE, Kurinji::kb_key_press_input_system.system())
            // mouse
            .add_system_to_stage(
                stage::UPDATE,
                Kurinji::mouse_button_press_input_system.system(),
            )
            .add_system_to_stage(stage::UPDATE, Kurinji::mouse_move_event_system.system());
    }
}

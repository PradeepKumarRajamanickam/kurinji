// publics
pub use self::{
    kurinji::Kurinji, axis::MouseAxis, axis::GamepadAxis, bindings::Bindings,
    event_phase::EventPhase, action_event::OnActionBegin,
    action_event::OnActionActive, action_event::OnActionProgress,
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
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum KurinjiStage {
    Reset,
    InputCapture,
    Event,
}

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
            .add_system_to_stage(
                CoreStage::PreUpdate,
                Kurinji::action_reset_system
                    .system()
                    .label(KurinjiStage::Reset),
            )
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .label(KurinjiStage::InputCapture)
                    .after(KurinjiStage::Reset)
                    .with_system(Kurinji::gamepad_event_system.system())
                    .with_system(
                        Kurinji::gamepad_button_press_input_system.system(),
                    )
                    .with_system(Kurinji::kb_key_press_input_system.system())
                    .with_system(
                        Kurinji::mouse_button_press_input_system.system(),
                    )
                    .with_system(Kurinji::mouse_move_event_system.system()),
            )
            .add_system_to_stage(
                CoreStage::PreUpdate,
                Kurinji::action_event_producer
                    .system()
                    .label(KurinjiStage::Event)
                    .after(KurinjiStage::InputCapture),
            );
    }
}

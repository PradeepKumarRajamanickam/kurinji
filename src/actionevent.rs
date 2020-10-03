use bevy_app::Events;
use bevy_ecs::{Res, ResMut};

use crate::inputmap::InputMap;

pub struct OnActionActive
{
    pub action: String,
    pub strength: f32
}

pub struct OnActionBegin
{
    pub action: String,
    pub strength: f32
}

pub struct OnActionProgress
{
    pub action: String,
    pub strength: f32
}

pub struct OnActionEnd
{
    pub action: String
}

impl InputMap {
    
    pub(crate) fn action_event_producer(
        input_map: Res<InputMap>,
        mut on_active_event: ResMut<Events<OnActionActive>>,
        mut on_begin_event: ResMut<Events<OnActionBegin>>,
        mut on_progress_event: ResMut<Events<OnActionProgress>>,
        mut on_end_event: ResMut<Events<OnActionEnd>>,
    )
    {
        for (action, strength) in input_map.action_raw_strength.clone()
        {
            if input_map.is_action_active(&action)
            {
                on_active_event.send(OnActionActive{ action: action.clone(), strength: strength});
            }

            if input_map.did_action_just_began(&action)
            {
                on_begin_event.send(OnActionBegin{ action: action.clone(), strength: strength});
            }

            if input_map.is_action_in_progress(&action)
            {
                on_progress_event.send(OnActionProgress{ action: action.clone(), strength: strength});
            }

            if input_map.did_action_just_end(&action)
            {
                on_end_event.send(OnActionEnd{ action: action.clone() });
            }
        }
    }
}
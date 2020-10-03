use std::fs;

use bevy::prelude::*;
use bevy_app::AppExit;
use bevy_app::Events;
use bevy_ecs::ResMut;
use bevy_prototype_input_map::{
    actionevent::OnActionActive, actionevent::OnActionEnd, inputmap::InputMap, InputMapPlugin,
};

#[derive(Default)]
pub struct ActionState {
    active_reader: EventReader<OnActionActive>,
    end_reader: EventReader<OnActionEnd>
}

fn main() {
    println!("Input Map How To Use Action Events");
    App::build()
        .add_default_plugins()
        // setup
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_active_events_system.system())
        .add_system(action_end_events_system.system())
        .run();
}

fn setup(mut input_map: ResMut<InputMap>) {
    let binding_json = fs::read_to_string("example/config/binding.json")
        .expect("Error! could not open config file");

    input_map.set_bindings_with_json(&binding_json);
}

fn action_end_events_system(
        mut state: Local<ActionState>,
        mut app_exit_events: ResMut<Events<AppExit>>,
        action_end_event: Res<Events<OnActionEnd>>,
    ) {
        if let Some(value) = state.end_reader.latest(&action_end_event) {
            if value.action == "QUIT_APP" {
                println!("Quiting...");
                app_exit_events.send(AppExit);
            }
        }
    }

fn action_active_events_system(
    mut state: Local<ActionState>,
    action_active_event: Res<Events<OnActionActive>>,
) {
    if let Some(value) = state.active_reader.latest(&action_active_event) {
        if value.action == "JUMP" {
            println!("Jumping...");
        }

        if value.action == "SHOOT" {
            println!("Bang");
        }

        if value.action == "AIM_UP" {
            println!("AIM_UP... [ strength: {}] ", value.strength);
        }

        if value.action == "AIM_DOWN" {
            println!("AIM_DOWN... [ strength: {}] ", value.strength);
        }

        if value.action == "AIM_LEFT" {
            println!("AIM_LEFT... [ strength: {}] ", value.strength);
        }

        if value.action == "AIM_RIGHT" {
            println!("AIM_RIGHT... [ strength: {}] ", value.strength);
        }
    }
}
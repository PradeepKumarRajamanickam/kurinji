use std::fs;

use bevy::prelude::*;
use bevy_app::AppExit;
use bevy_app::Events;
use bevy_ecs::ResMut;
use bevy_prototype_input_map::*;

fn main() {
    println!("Input Map Binding From Json Example");
    App::build()
        .add_default_plugins()
        // setup
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_system.system())
        .run();
}

fn setup(mut input_map: ResMut<InputMap>) {
    let binding_json = fs::read_to_string("example/config/binding.json")
        .expect("Error! could not open config file");

    input_map.set_bindings_with_json(&binding_json);
}

fn action_system(input_map: Res<InputMap>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if input_map.is_action_active("BACK_PLAYER1") {
        println!("Player 1 wants to go back");
    }
    if input_map.is_action_active("BACK_PLAYER2") {
        println!("Player 2 wants to go back");
    }

    if input_map.is_action_active("JUMP") {
        println!("Jumping...");
    }

    if input_map.is_action_active("SHOOT") {
        println!("Bang");
    }

    if input_map.is_action_active("AIM_UP") {
        println!(
            "AIM_UP... [ strength: {}] ",
            input_map.get_action_strength("AIM_UP")
        );
    }

    if input_map.is_action_active("AIM_DOWN") {
        println!(
            "AIM_DOWN... [ strength: {}] ",
            input_map.get_action_strength("AIM_DOWN")
        );
    }

    if input_map.is_action_active("AIM_LEFT") {
        println!(
            "AIM_LEFT... [ strength: {}] ",
            input_map.get_action_strength("AIM_LEFT")
        );
    }

    if input_map.is_action_active("AIM_RIGHT") {
        println!(
            "AIM_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("AIM_RIGHT")
        );
    }

    if input_map.is_action_active("MOVE_LEFT") {
        println!(
            "MOVE_LEFT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_LEFT")
        );
    }

    if input_map.is_action_active("MOVE_RIGHT") {
        println!(
            "MOVE_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_RIGHT")
        );
    }

    if input_map.is_action_active("MOVE_FORWARD") {
        println!(
            "MOVE_FORWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_FORWARD")
        );
    }

    if input_map.is_action_active("MOVE_BACKWARD") {
        println!(
            "MOVE_BACKWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_BACKWARD")
        );
    }

    if input_map.is_action_active("QUIT_APP") {
        println!("Quiting...");
        app_exit_events.send(AppExit);
    }
}

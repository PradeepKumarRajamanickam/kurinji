use std::fs;

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::ecs::ResMut;
use bevy_prototype_input_map::*;

fn main() {
    println!("Input Map Binding Gamepad From RON Example");
    App::build()
        .add_default_plugins()
        // setup
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_system.system())
        .run();
}

fn setup(mut input_map: ResMut<InputMap>) {
    let binding_ron = fs::read_to_string("example/config/gamepad.ron")
        .expect("Error! could not open config file");

    input_map
        .set_bindings_with_ron(&binding_ron)
        // custom strength curve function
        .set_strength_curve_function("AIM_UP_PLYR1", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_UP_PLYR2", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_UP_PLYR3", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_UP_PLYR4", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_DOWN_PLYR1", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_DOWN_PLYR2", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_DOWN_PLYR3", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_DOWN_PLYR4", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_LEFT_PLYR1", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_LEFT_PLYR2", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_LEFT_PLYR3", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_LEFT_PLYR4", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_RIGHT_PLYR1", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_RIGHT_PLYR2", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_RIGHT_PLYR3", |x| -> f32 { x.powi(2) })
        .set_strength_curve_function("AIM_RIGHT_PLYR4", |x| -> f32 { x.powi(2) });
}

fn action_system(input_map: Res<InputMap>, mut app_exit_events: ResMut<Events<AppExit>>) {
    // PLAYER 1
    if input_map.is_action_active("BACK_PLYR1") {
        println!("Player 1 wants to go back");
    }

    if input_map.is_action_active("JUMP_PLYR1") {
        println!("Player 1 Jumping...");
    }

    if input_map.is_action_active("SHOOT_PLYR1") {
        println!("Player 1 Bang");
    }

    if input_map.is_action_active("AIM_UP_PLYR1") {
        println!(
            "Player 1 AIM_UP... [ strength: {}] ",
            input_map.get_action_strength("AIM_UP_PLYR1")
        );
    }

    if input_map.is_action_active("AIM_DOWN_PLYR1") {
        println!(
            "Player 1 AIM_DOWN... [ strength: {}] ",
            input_map.get_action_strength("AIM_DOWN_PLYR1")
        );
    }

    if input_map.is_action_active("AIM_LEFT_PLYR1") {
        println!(
            "Player 1 AIM_LEFT... [ strength: {}] ",
            input_map.get_action_strength("AIM_LEFT_PLYR1")
        );
    }

    if input_map.is_action_active("AIM_RIGHT_PLYR1") {
        println!(
            "Player 1 AIM_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("AIM_RIGHT_PLYR1")
        );
    }

    if input_map.is_action_active("MOVE_LEFT_PLYR1") {
        println!(
            "Player 1 MOVE_LEFT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_LEFT_PLYR1")
        );
    }

    if input_map.is_action_active("MOVE_RIGHT_PLYR1") {
        println!(
            "Player 1 MOVE_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_RIGHT_PLYR1")
        );
    }

    if input_map.is_action_active("MOVE_FORWARD_PLYR1") {
        println!(
            "Player 1 MOVE_FORWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_FORWARD_PLYR1")
        );
    }

    if input_map.is_action_active("MOVE_BACKWARD_PLYR1") {
        println!(
            "Player 1 MOVE_BACKWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_BACKWARD_PLYR1")
        );
    }

    if input_map.is_action_active("QUIT_APP_PLYR1") {
        println!("Player 1 Quiting...");
        app_exit_events.send(AppExit);
    }

    // PLAYER 2
    if input_map.is_action_active("BACK_PLYR2") {
        println!("Player 2 wants to go back");
    }

    if input_map.is_action_active("JUMP_PLYR2") {
        println!("Player 2 Jumping...");
    }

    if input_map.is_action_active("SHOOT_PLYR2") {
        println!("Player 2 Bang");
    }

    if input_map.is_action_active("AIM_UP_PLYR2") {
        println!(
            "Player 2 AIM_UP... [ strength: {}] ",
            input_map.get_action_strength("AIM_UP_PLYR2")
        );
    }

    if input_map.is_action_active("AIM_DOWN_PLYR2") {
        println!(
            "Player 2 AIM_DOWN... [ strength: {}] ",
            input_map.get_action_strength("AIM_DOWN_PLYR2")
        );
    }

    if input_map.is_action_active("AIM_LEFT_PLYR2") {
        println!(
            "Player 2 AIM_LEFT... [ strength: {}] ",
            input_map.get_action_strength("AIM_LEFT_PLYR2")
        );
    }

    if input_map.is_action_active("AIM_RIGHT_PLYR2") {
        println!(
            "Player 2 AIM_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("AIM_RIGHT_PLYR2")
        );
    }

    if input_map.is_action_active("MOVE_LEFT_PLYR2") {
        println!(
            "Player 2 MOVE_LEFT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_LEFT_PLYR2")
        );
    }

    if input_map.is_action_active("MOVE_RIGHT_PLYR2") {
        println!(
            "Player 2 MOVE_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_RIGHT_PLYR2")
        );
    }

    if input_map.is_action_active("MOVE_FORWARD_PLYR2") {
        println!(
            "Player 2 MOVE_FORWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_FORWARD_PLYR2")
        );
    }

    if input_map.is_action_active("MOVE_BACKWARD_PLYR2") {
        println!(
            "Player 2 MOVE_BACKWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_BACKWARD_PLYR2")
        );
    }

    if input_map.is_action_active("QUIT_APP_PLYR2") {
        println!("Player 2 Quiting...");
        app_exit_events.send(AppExit);
    }

    // PLAYER 3
    if input_map.is_action_active("BACK_PLYR3") {
        println!("Player 3 wants to go back");
    }

    if input_map.is_action_active("JUMP_PLYR3") {
        println!("Player 3 Jumping...");
    }

    if input_map.is_action_active("SHOOT_PLYR3") {
        println!("Player 3 Bang");
    }

    if input_map.is_action_active("AIM_UP_PLYR3") {
        println!(
            "Player 3 AIM_UP... [ strength: {}] ",
            input_map.get_action_strength("AIM_UP_PLYR3")
        );
    }

    if input_map.is_action_active("AIM_DOWN_PLYR3") {
        println!(
            "Player 3 AIM_DOWN... [ strength: {}] ",
            input_map.get_action_strength("AIM_DOWN_PLYR3")
        );
    }

    if input_map.is_action_active("AIM_LEFT_PLYR3") {
        println!(
            "Player 3 AIM_LEFT... [ strength: {}] ",
            input_map.get_action_strength("AIM_LEFT_PLYR3")
        );
    }

    if input_map.is_action_active("AIM_RIGHT_PLYR3") {
        println!(
            "Player 3 AIM_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("AIM_RIGHT_PLYR3")
        );
    }

    if input_map.is_action_active("MOVE_LEFT_PLYR3") {
        println!(
            "Player 3 MOVE_LEFT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_LEFT_PLYR3")
        );
    }

    if input_map.is_action_active("MOVE_RIGHT_PLYR3") {
        println!(
            "Player 3 MOVE_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_RIGHT_PLYR3")
        );
    }

    if input_map.is_action_active("MOVE_FORWARD_PLYR3") {
        println!(
            "Player 3 MOVE_FORWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_FORWARD_PLYR3")
        );
    }

    if input_map.is_action_active("MOVE_BACKWARD_PLYR3") {
        println!(
            "Player 3 MOVE_BACKWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_BACKWARD_PLYR3")
        );
    }

    if input_map.is_action_active("QUIT_APP_PLYR3") {
        println!("Player 3 Quiting...");
        app_exit_events.send(AppExit);
    }

    // PLAYER 4
    if input_map.is_action_active("BACK_PLYR4") {
        println!("Player 4 wants to go back");
    }

    if input_map.is_action_active("JUMP_PLYR4") {
        println!("Player 4 Jumping...");
    }

    if input_map.is_action_active("SHOOT_PLYR4") {
        println!("Player 4 Bang");
    }

    if input_map.is_action_active("AIM_UP_PLYR4") {
        println!(
            "Player 4 AIM_UP... [ strength: {}] ",
            input_map.get_action_strength("AIM_UP_PLYR4")
        );
    }

    if input_map.is_action_active("AIM_DOWN_PLYR4") {
        println!(
            "Player 4 AIM_DOWN... [ strength: {}] ",
            input_map.get_action_strength("AIM_DOWN_PLYR4")
        );
    }

    if input_map.is_action_active("AIM_LEFT_PLYR4") {
        println!(
            "Player 4 AIM_LEFT... [ strength: {}] ",
            input_map.get_action_strength("AIM_LEFT_PLYR4")
        );
    }

    if input_map.is_action_active("AIM_RIGHT_PLYR4") {
        println!(
            "Player 4 AIM_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("AIM_RIGHT_PLYR4")
        );
    }

    if input_map.is_action_active("MOVE_LEFT_PLYR4") {
        println!(
            "Player 4 MOVE_LEFT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_LEFT_PLYR4")
        );
    }

    if input_map.is_action_active("MOVE_RIGHT_PLYR4") {
        println!(
            "Player 4 MOVE_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("MOVE_RIGHT_PLYR4")
        );
    }

    if input_map.is_action_active("MOVE_FORWARD_PLYR4") {
        println!(
            "Player 4 MOVE_FORWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_FORWARD_PLYR4")
        );
    }

    if input_map.is_action_active("MOVE_BACKWARD_PLYR4") {
        println!(
            "Player 4 MOVE_BACKWARD... [ strength: {}] ",
            input_map.get_action_strength("MOVE_BACKWARD_PLYR4")
        );
    }

    if input_map.is_action_active("QUIT_APP_PLYR4") {
        println!("Player 4 Quiting...");
        app_exit_events.send(AppExit);
    }
}

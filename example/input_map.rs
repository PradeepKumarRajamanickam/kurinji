use bevy::prelude::*;
use bevy_app::AppExit;
use bevy_app::Events;
use bevy_ecs::ResMut;
use bevy_prototype_input_map::{inputmap::InputMap, InputMapPlugin, axis::Axis};

fn main() {
    App::build()
        .add_default_plugins()
        // setup
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_system.system())
        .run();
}

fn setup(
    mut input_map: ResMut<InputMap>,
) {
    // keyboard
    input_map.bind_keyboard_pressed(KeyCode::Space, "JUMP");
    input_map.bind_keyboard_pressed(KeyCode::Return, "SHOOT");

    input_map.bind_keyboard_pressed(KeyCode::Escape, "QUIT_APP");

    // // mouse
    input_map.bind_mouse_button_pressed(MouseButton::Left, "SHOOT");
    input_map.bind_mouse_button_pressed(MouseButton::Right, "JUMP");

    input_map.bind_mouse_motion(Axis::YNegative, "AIM_UP");
    input_map.bind_mouse_motion(Axis::YPositive, "AIM_DOWN");
    input_map.bind_mouse_motion(Axis::XNegative, "AIM_LEFT");
    input_map.bind_mouse_motion(Axis::XPositive, "AIM_RIGHT");

    // dead zone
    input_map.set_dead_zone("AIM_UP", 0.1);
    input_map.set_dead_zone("AIM_DOWN", 0.1);
    input_map.set_dead_zone("AIM_LEFT", 0.1);
    input_map.set_dead_zone("AIM_RIGHT", 0.1);

    // strength curve function
    input_map.set_strength_curve_function("AIM_UP", |x  | -> f32 { x.powi(2)});
    input_map.set_strength_curve_function("AIM_DOWN", |x  | -> f32 { x.powi(2) });
    input_map.set_strength_curve_function("AIM_LEFT", |x  | -> f32 { x.powi(2) });
    input_map.set_strength_curve_function("AIM_RIGHT", |x  | -> f32 { x.powi(2) });
}

/// This system prints 'A' key state
fn action_system(input_map: Res<InputMap>, mut app_exit_events: ResMut<Events<AppExit>>) {
    if input_map.is_action_in_progress("JUMP") {
        println!("Jumping...");
    }

    if input_map.is_action_in_progress("SHOOT") {
        println!("Bang");
    }

    if input_map.is_action_in_progress("AIM_UP") {
        println!(
            "AIM_UP... [ strength: {}] ",
            input_map.get_action_strength("AIM_UP")
        );
    }

    if input_map.is_action_in_progress("AIM_DOWN") {
        println!(
            "AIM_DOWN... [ strength: {}] ",
            input_map.get_action_strength("AIM_DOWN")
        );
    }

    if input_map.is_action_in_progress("AIM_LEFT") {
        println!(
            "AIM_LEFT... [ strength: {}] ",
            input_map.get_action_strength("AIM_LEFT")
        );
    }

    if input_map.is_action_in_progress("AIM_RIGHT") {
        println!(
            "AIM_RIGHT... [ strength: {}] ",
            input_map.get_action_strength("AIM_RIGHT")
        );
    }

    if input_map.is_action_in_progress("QUIT_APP") {
        println!("Quiting...");
        app_exit_events.send(AppExit);
    }
}

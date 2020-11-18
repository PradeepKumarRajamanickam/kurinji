use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::ecs::ResMut;
use bevy_prototype_input_map::*;

fn main() {
    println!("Input Map Binding In Code Example");
    App::build()
        .add_plugins(DefaultPlugins)
        // setup
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_system.system())
        .run();
}

fn setup(mut input_map: ResMut<InputMap>) {
    input_map
        // keyboard
        .bind_keyboard_pressed(KeyCode::Space, "JUMP")
        .bind_keyboard_pressed(KeyCode::Return, "SHOOT")
        .bind_keyboard_pressed(KeyCode::A, "MOVE_LEFT")
        .bind_keyboard_pressed(KeyCode::D, "MOVE_RIGHT")
        .bind_keyboard_pressed(KeyCode::W, "MOVE_FORWARD")
        .bind_keyboard_pressed(KeyCode::S, "MOVE_BACKWARD")
        .bind_keyboard_pressed(KeyCode::Escape, "QUIT_APP")
        // mouse
        .bind_mouse_button_pressed(MouseButton::Left, "SHOOT")
        .bind_mouse_button_pressed(MouseButton::Right, "JUMP")
        .bind_mouse_motion(MouseAxis::YNegative, "AIM_UP")
        .bind_mouse_motion(MouseAxis::YPositive, "AIM_DOWN")
        .bind_mouse_motion(MouseAxis::XNegative, "AIM_LEFT")
        .bind_mouse_motion(MouseAxis::XPositive, "AIM_RIGHT")
        // set event phase
        .set_event_phase("QUIT_APP", EventPhase::OnEnded)
        .set_event_phase("SHOOT", EventPhase::OnBegin);

    println!("{}", input_map.get_bindings_as_json().unwrap());
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

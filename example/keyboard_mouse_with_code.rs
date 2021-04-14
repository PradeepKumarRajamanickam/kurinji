use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::ecs::system::ResMut;
use kurinji::{EventPhase, MouseAxis, Kurinji, KurinjiPlugin};
fn main() {
    println!("Kurinji Binding In Code Example");
    App::build()
        .add_plugins(DefaultPlugins)
        // setup
        .add_plugin(KurinjiPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_system.system())
        .run();
}
fn setup(mut kurinji: ResMut<Kurinji>) {
    kurinji
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
    println!("{}", kurinji.get_bindings_as_json().unwrap());
}
fn action_system(
    kurinji: Res<Kurinji>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if kurinji.is_action_active("BACK_PLAYER1") {
        println!("Player 1 wants to go back");
    }
    if kurinji.is_action_active("BACK_PLAYER2") {
        println!("Player 2 wants to go back");
    }
    if kurinji.is_action_active("JUMP") {
        println!("Jumping...");
    }
    if kurinji.is_action_active("SHOOT") {
        println!("Bang");
    }
    if kurinji.is_action_active("AIM_UP") {
        println!(
            "AIM_UP... [ strength: {}] ",
            kurinji.get_action_strength("AIM_UP")
        );
    }
    if kurinji.is_action_active("AIM_DOWN") {
        println!(
            "AIM_DOWN... [ strength: {}] ",
            kurinji.get_action_strength("AIM_DOWN")
        );
    }
    if kurinji.is_action_active("AIM_LEFT") {
        println!(
            "AIM_LEFT... [ strength: {}] ",
            kurinji.get_action_strength("AIM_LEFT")
        );
    }
    if kurinji.is_action_active("AIM_RIGHT") {
        println!(
            "AIM_RIGHT... [ strength: {}] ",
            kurinji.get_action_strength("AIM_RIGHT")
        );
    }
    if kurinji.is_action_active("MOVE_LEFT") {
        println!(
            "MOVE_LEFT... [ strength: {}] ",
            kurinji.get_action_strength("MOVE_LEFT")
        );
    }
    if kurinji.is_action_active("MOVE_RIGHT") {
        println!(
            "MOVE_RIGHT... [ strength: {}] ",
            kurinji.get_action_strength("MOVE_RIGHT")
        );
    }
    if kurinji.is_action_active("MOVE_FORWARD") {
        println!(
            "MOVE_FORWARD... [ strength: {}] ",
            kurinji.get_action_strength("MOVE_FORWARD")
        );
    }
    if kurinji.is_action_active("MOVE_BACKWARD") {
        println!(
            "MOVE_BACKWARD... [ strength: {}] ",
            kurinji.get_action_strength("MOVE_BACKWARD")
        );
    }
    if kurinji.is_action_active("QUIT_APP") {
        println!("Quiting...");
        app_exit_events.send(AppExit);
    }
}

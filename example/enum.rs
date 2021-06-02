use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::ecs::ResMut;
use kurinji::{EventPhase, Kurinji, KurinjiPlugin, MouseAxis, OnActionActive};
use strum::{IntoStaticStr, EnumString};
#[derive(IntoStaticStr, EnumString, PartialEq, Eq)]
enum Action {
    JUMP,
    SHOOT,
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
    QuitApp,
    AimUp,
    AimDown,
    AimLeft,
    AimRight,
}
fn main() {
    println!("Kurinji Binding In Code Example");
    App::build()
        .add_plugins(DefaultPlugins)
        // setup
        .add_plugin(KurinjiPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_system.system())
        .add_system(action_active_events_system.system())
        .run();
}
fn setup(mut kurinji: ResMut<Kurinji>) {
    kurinji
        // keyboard
        .bind_keyboard_pressed(KeyCode::Space, Action::JUMP)
        .bind_keyboard_pressed(KeyCode::Return, Action::SHOOT)
        .bind_keyboard_pressed(KeyCode::A, Action::MoveLeft)
        .bind_keyboard_pressed(KeyCode::D, Action::MoveRight)
        .bind_keyboard_pressed(KeyCode::W, Action::MoveForward)
        .bind_keyboard_pressed(KeyCode::S, Action::MoveBackward)
        .bind_keyboard_pressed(KeyCode::Escape, Action::QuitApp)
        // mouse
        .bind_mouse_button_pressed(MouseButton::Left, Action::SHOOT)
        .bind_mouse_button_pressed(MouseButton::Right, Action::JUMP)
        .bind_mouse_motion(MouseAxis::YNegative, Action::AimUp)
        .bind_mouse_motion(MouseAxis::YPositive, Action::AimDown)
        .bind_mouse_motion(MouseAxis::XNegative, Action::AimLeft)
        .bind_mouse_motion(MouseAxis::XPositive, Action::AimRight)
        // set event phase
        .set_event_phase(Action::QuitApp, EventPhase::OnEnded)
        .set_event_phase(Action::SHOOT, EventPhase::OnBegin);
    println!("{}", kurinji.get_bindings_as_json().unwrap());
}
fn action_system(
    kurinji: Res<Kurinji>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if kurinji.is_action_active(Action::JUMP) {
        println!("Jumping...");
    }
    if kurinji.is_action_active(Action::SHOOT) {
        println!("Bang");
    }
    if kurinji.is_action_active(Action::AimUp) {
        println!(
            "AIM_UP... [ strength: {}] ",
            kurinji.get_action_strength(Action::AimUp)
        );
    }
    if kurinji.is_action_active(Action::AimDown) {
        println!(
            "AIM_DOWN... [ strength: {}] ",
            kurinji.get_action_strength(Action::AimDown)
        );
    }
    if kurinji.is_action_active(Action::AimLeft) {
        println!(
            "AIM_LEFT... [ strength: {}] ",
            kurinji.get_action_strength(Action::AimLeft)
        );
    }
    if kurinji.is_action_active(Action::AimRight) {
        println!(
            "AIM_RIGHT... [ strength: {}] ",
            kurinji.get_action_strength(Action::AimRight)
        );
    }
    if kurinji.is_action_active(Action::MoveLeft) {
        println!(
            "MOVE_LEFT... [ strength: {}] ",
            kurinji.get_action_strength(Action::MoveLeft)
        );
    }
    if kurinji.is_action_active(Action::MoveRight) {
        println!(
            "MOVE_RIGHT... [ strength: {}] ",
            kurinji.get_action_strength(Action::MoveRight)
        );
    }
    if kurinji.is_action_active(Action::MoveForward) {
        println!(
            "MOVE_FORWARD... [ strength: {}] ",
            kurinji.get_action_strength(Action::MoveForward)
        );
    }
    if kurinji.is_action_active(Action::MoveBackward) {
        println!(
            "MOVE_BACKWARD... [ strength: {}] ",
            kurinji.get_action_strength(Action::MoveBackward)
        );
    }
    if kurinji.is_action_active(Action::QuitApp) {
        println!("Quiting...");
        app_exit_events.send(AppExit);
    }
}
fn action_active_events_system(
    action_active_event: Res<Events<OnActionActive>>,
) {
    for value in action_active_event.iter_current_update_events() {
        if value.action() == Some(Action::JUMP) {
            println!("Jumping...");
        }
        if value.action() == Some(Action::SHOOT) {
            println!("Bang");
        }
        if value.action() == Some(Action::AimUp) {
            println!("AIM_UP... [ strength: {}] ", value.strength);
        }
        if value.action() == Some(Action::AimDown) {
            println!("AIM_DOWN... [ strength: {}] ", value.strength);
        }
        if value.action() == Some(Action::AimLeft) {
            println!("AIM_LEFT... [ strength: {}] ", value.strength);
        }
        if value.action() == Some(Action::AimRight) {
            println!("AIM_RIGHT... [ strength: {}] ", value.strength);
        }
    }
}

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::ecs::ResMut;
use kurinji::{
    EventPhase,
    GamepadAxis,
    Kurinji,
    KurinjiPlugin,
};
fn main()
{
    println!("Kurinji Binding Gamepad In Code Example");
    App::build().add_plugins(DefaultPlugins)
                // setup
                .add_plugin(KurinjiPlugin::default())
                .add_startup_system(setup.system())
                .add_system(action_system.system())
                .run();
}
fn setup(mut kurinji: ResMut<Kurinji>)
{
    kurinji
        // PLAYER 1
        .bind_gamepad_button_pressed_for_player(
            0,
            GamepadButtonType::Select,
            "BACK_PLYR1",
        )
        .bind_gamepad_button_pressed_for_player(
            0,
            GamepadButtonType::Start,
            "QUIT_APP_PLYR1",
        )
        .bind_gamepad_button_pressed_for_player(
            0,
            GamepadButtonType::South,
            "SHOOT_PLYR1",
        )
        .bind_gamepad_button_pressed_for_player(
            0,
            GamepadButtonType::RightTrigger2,
            "SHOOT_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::RightStickXNegative,
            "AIM_LEFT_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::RightStickXPositive,
            "AIM_RIGHT_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::RightStickYPositive,
            "AIM_UP_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::RightStickYNegative,
            "AIM_DOWN_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::LeftStickXNegative,
            "MOVE_LEFT_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::LeftStickXPositive,
            "MOVE_RIGHT_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::LeftStickYPositive,
            "MOVE_FORWARD_PLYR1",
        )
        .bind_gamepad_axis_for_player(
            0,
            GamepadAxis::LeftStickYNegative,
            "MOVE_BACKWARD_PLYR1",
        )
        // PLAYER 2
        .bind_gamepad_button_pressed_for_player(
            1,
            GamepadButtonType::Select,
            "BACK_PLYR2",
        )
        .bind_gamepad_button_pressed_for_player(
            1,
            GamepadButtonType::Start,
            "QUIT_APP_PLYR2",
        )
        .bind_gamepad_button_pressed_for_player(
            1,
            GamepadButtonType::South,
            "SHOOT_PLYR2",
        )
        .bind_gamepad_button_pressed_for_player(
            1,
            GamepadButtonType::RightTrigger2,
            "SHOOT_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::RightStickXNegative,
            "AIM_LEFT_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::RightStickXPositive,
            "AIM_RIGHT_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::RightStickYPositive,
            "AIM_UP_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::RightStickYNegative,
            "AIM_DOWN_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::LeftStickXNegative,
            "MOVE_LEFT_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::LeftStickXPositive,
            "MOVE_RIGHT_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::LeftStickYPositive,
            "MOVE_FORWARD_PLYR2",
        )
        .bind_gamepad_axis_for_player(
            1,
            GamepadAxis::LeftStickYNegative,
            "MOVE_BACKWARD_PLYR2",
        )
        // PLAYER 3
        .bind_gamepad_button_pressed_for_player(
            2,
            GamepadButtonType::Select,
            "BACK_PLYR3",
        )
        .bind_gamepad_button_pressed_for_player(
            2,
            GamepadButtonType::Start,
            "QUIT_APP_PLYR3",
        )
        .bind_gamepad_button_pressed_for_player(
            2,
            GamepadButtonType::South,
            "SHOOT_PLYR3",
        )
        .bind_gamepad_button_pressed_for_player(
            2,
            GamepadButtonType::RightTrigger2,
            "SHOOT_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::RightStickXNegative,
            "AIM_LEFT_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::RightStickXPositive,
            "AIM_RIGHT_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::RightStickYPositive,
            "AIM_UP_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::RightStickYNegative,
            "AIM_DOWN_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::LeftStickXNegative,
            "MOVE_LEFT_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::LeftStickXPositive,
            "MOVE_RIGHT_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::LeftStickYPositive,
            "MOVE_FORWARD_PLYR3",
        )
        .bind_gamepad_axis_for_player(
            2,
            GamepadAxis::LeftStickYNegative,
            "MOVE_BACKWARD_PLYR3",
        )
        // PLAYER 4
        .bind_gamepad_button_pressed_for_player(
            3,
            GamepadButtonType::Select,
            "BACK_PLYR4",
        )
        .bind_gamepad_button_pressed_for_player(
            3,
            GamepadButtonType::Start,
            "QUIT_APP_PLYR4",
        )
        .bind_gamepad_button_pressed_for_player(
            3,
            GamepadButtonType::South,
            "SHOOT_PLYR4",
        )
        .bind_gamepad_button_pressed_for_player(
            3,
            GamepadButtonType::RightTrigger2,
            "SHOOT_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::RightStickXNegative,
            "AIM_LEFT_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::RightStickXPositive,
            "AIM_RIGHT_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::RightStickYPositive,
            "AIM_UP_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::RightStickYNegative,
            "AIM_DOWN_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::LeftStickXNegative,
            "MOVE_LEFT_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::LeftStickXPositive,
            "MOVE_RIGHT_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::LeftStickYPositive,
            "MOVE_FORWARD_PLYR4",
        )
        .bind_gamepad_axis_for_player(
            3,
            GamepadAxis::LeftStickYNegative,
            "MOVE_BACKWARD_PLYR4",
        )
        // set event phase
        .set_event_phase("BACK_PLYR1", EventPhase::OnEnded)
        .set_event_phase("BACK_PLYR2", EventPhase::OnEnded)
        .set_event_phase("BACK_PLYR3", EventPhase::OnEnded)
        .set_event_phase("BACK_PLYR4", EventPhase::OnEnded)
        .set_event_phase("SHOOT_PLYR1", EventPhase::OnBegin)
        .set_event_phase("SHOOT_PLYR2", EventPhase::OnBegin)
        .set_event_phase("SHOOT_PLYR3", EventPhase::OnBegin)
        .set_event_phase("SHOOT_PLYR4", EventPhase::OnBegin)
        .set_event_phase("QUIT_APP_PLYR1", EventPhase::OnEnded)
        .set_event_phase("QUIT_APP_PLYR2", EventPhase::OnEnded)
        .set_event_phase("QUIT_APP_PLYR3", EventPhase::OnEnded)
        .set_event_phase("QUIT_APP_PLYR4", EventPhase::OnEnded)
        // dead zone
        .set_dead_zone("AIM_UP_PLYR1", 0.1)
        .set_dead_zone("AIM_UP_PLYR2", 0.1)
        .set_dead_zone("AIM_UP_PLYR3", 0.1)
        .set_dead_zone("AIM_UP_PLYR4", 0.1)
        .set_dead_zone("AIM_DOWN_PLYR1", 0.1)
        .set_dead_zone("AIM_DOWN_PLYR2", 0.1)
        .set_dead_zone("AIM_DOWN_PLYR3", 0.1)
        .set_dead_zone("AIM_DOWN_PLYR4", 0.1)
        .set_dead_zone("AIM_LEFT_PLYR1", 0.1)
        .set_dead_zone("AIM_LEFT_PLYR2", 0.1)
        .set_dead_zone("AIM_LEFT_PLYR3", 0.1)
        .set_dead_zone("AIM_LEFT_PLYR4", 0.1)
        .set_dead_zone("AIM_RIGHT_PLYR1", 0.1)
        .set_dead_zone("AIM_RIGHT_PLYR2", 0.1)
        .set_dead_zone("AIM_RIGHT_PLYR3", 0.1)
        .set_dead_zone("AIM_RIGHT_PLYR4", 0.1)
        .set_dead_zone("MOVE_LEFT_PLYR1", 0.25)
        .set_dead_zone("MOVE_LEFT_PLYR2", 0.25)
        .set_dead_zone("MOVE_LEFT_PLYR3", 0.25)
        .set_dead_zone("MOVE_LEFT_PLYR4", 0.25)
        .set_dead_zone("MOVE_RIGHT_PLYR1", 0.25)
        .set_dead_zone("MOVE_RIGHT_PLYR2", 0.25)
        .set_dead_zone("MOVE_RIGHT_PLYR3", 0.25)
        .set_dead_zone("MOVE_RIGHT_PLYR4", 0.25)
        .set_dead_zone("MOVE_FORWARD_PLYR1", 0.25)
        .set_dead_zone("MOVE_FORWARD_PLYR2", 0.25)
        .set_dead_zone("MOVE_FORWARD_PLYR3", 0.25)
        .set_dead_zone("MOVE_FORWARD_PLYR4", 0.25)
        .set_dead_zone("MOVE_BACKWARD_PLYR1", 0.25)
        .set_dead_zone("MOVE_BACKWARD_PLYR2", 0.25)
        .set_dead_zone("MOVE_BACKWARD_PLYR3", 0.25)
        .set_dead_zone("MOVE_BACKWARD_PLYR4", 0.25)
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
        .set_strength_curve_function("AIM_RIGHT_PLYR1", |x| -> f32 {
            x.powi(2)
        })
        .set_strength_curve_function("AIM_RIGHT_PLYR2", |x| -> f32 {
            x.powi(2)
        })
        .set_strength_curve_function("AIM_RIGHT_PLYR3", |x| -> f32 {
            x.powi(2)
        })
        .set_strength_curve_function("AIM_RIGHT_PLYR4", |x| -> f32 {
            x.powi(2)
        });
    println!("{}", kurinji.get_bindings_as_json().unwrap());
}
fn action_system(kurinji: Res<Kurinji>,
                 mut app_exit_events: ResMut<Events<AppExit>>)
{
    // PLAYER 1
    if kurinji.is_action_active("BACK_PLYR1")
    {
        println!("Player 1 wants to go back");
    }
    if kurinji.is_action_active("JUMP_PLYR1")
    {
        println!("Player 1 Jumping...");
    }
    if kurinji.is_action_active("SHOOT_PLYR1")
    {
        println!("Player 1 Bang");
    }
    if kurinji.is_action_active("AIM_UP_PLYR1")
    {
        println!("Player 1 AIM_UP... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_UP_PLYR1"));
    }
    if kurinji.is_action_active("AIM_DOWN_PLYR1")
    {
        println!("Player 1 AIM_DOWN... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_DOWN_PLYR1"));
    }
    if kurinji.is_action_active("AIM_LEFT_PLYR1")
    {
        println!("Player 1 AIM_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_LEFT_PLYR1"));
    }
    if kurinji.is_action_active("AIM_RIGHT_PLYR1")
    {
        println!("Player 1 AIM_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_RIGHT_PLYR1"));
    }
    if kurinji.is_action_active("MOVE_LEFT_PLYR1")
    {
        println!("Player 1 MOVE_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_LEFT_PLYR1"));
    }
    if kurinji.is_action_active("MOVE_RIGHT_PLYR1")
    {
        println!("Player 1 MOVE_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_RIGHT_PLYR1"));
    }
    if kurinji.is_action_active("MOVE_FORWARD_PLYR1")
    {
        println!("Player 1 MOVE_FORWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_FORWARD_PLYR1"));
    }
    if kurinji.is_action_active("MOVE_BACKWARD_PLYR1")
    {
        println!("Player 1 MOVE_BACKWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_BACKWARD_PLYR1"));
    }
    if kurinji.is_action_active("QUIT_APP_PLYR1")
    {
        println!("Player 1 Quiting...");
        app_exit_events.send(AppExit);
    }
    // PLAYER 2
    if kurinji.is_action_active("BACK_PLYR2")
    {
        println!("Player 2 wants to go back");
    }
    if kurinji.is_action_active("JUMP_PLYR2")
    {
        println!("Player 2 Jumping...");
    }
    if kurinji.is_action_active("SHOOT_PLYR2")
    {
        println!("Player 2 Bang");
    }
    if kurinji.is_action_active("AIM_UP_PLYR2")
    {
        println!("Player 2 AIM_UP... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_UP_PLYR2"));
    }
    if kurinji.is_action_active("AIM_DOWN_PLYR2")
    {
        println!("Player 2 AIM_DOWN... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_DOWN_PLYR2"));
    }
    if kurinji.is_action_active("AIM_LEFT_PLYR2")
    {
        println!("Player 2 AIM_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_LEFT_PLYR2"));
    }
    if kurinji.is_action_active("AIM_RIGHT_PLYR2")
    {
        println!("Player 2 AIM_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_RIGHT_PLYR2"));
    }
    if kurinji.is_action_active("MOVE_LEFT_PLYR2")
    {
        println!("Player 2 MOVE_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_LEFT_PLYR2"));
    }
    if kurinji.is_action_active("MOVE_RIGHT_PLYR2")
    {
        println!("Player 2 MOVE_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_RIGHT_PLYR2"));
    }
    if kurinji.is_action_active("MOVE_FORWARD_PLYR2")
    {
        println!("Player 2 MOVE_FORWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_FORWARD_PLYR2"));
    }
    if kurinji.is_action_active("MOVE_BACKWARD_PLYR2")
    {
        println!("Player 2 MOVE_BACKWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_BACKWARD_PLYR2"));
    }
    if kurinji.is_action_active("QUIT_APP_PLYR2")
    {
        println!("Player 2 Quiting...");
        app_exit_events.send(AppExit);
    }
    // PLAYER 3
    if kurinji.is_action_active("BACK_PLYR3")
    {
        println!("Player 3 wants to go back");
    }
    if kurinji.is_action_active("JUMP_PLYR3")
    {
        println!("Player 3 Jumping...");
    }
    if kurinji.is_action_active("SHOOT_PLYR3")
    {
        println!("Player 3 Bang");
    }
    if kurinji.is_action_active("AIM_UP_PLYR3")
    {
        println!("Player 3 AIM_UP... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_UP_PLYR3"));
    }
    if kurinji.is_action_active("AIM_DOWN_PLYR3")
    {
        println!("Player 3 AIM_DOWN... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_DOWN_PLYR3"));
    }
    if kurinji.is_action_active("AIM_LEFT_PLYR3")
    {
        println!("Player 3 AIM_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_LEFT_PLYR3"));
    }
    if kurinji.is_action_active("AIM_RIGHT_PLYR3")
    {
        println!("Player 3 AIM_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_RIGHT_PLYR3"));
    }
    if kurinji.is_action_active("MOVE_LEFT_PLYR3")
    {
        println!("Player 3 MOVE_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_LEFT_PLYR3"));
    }
    if kurinji.is_action_active("MOVE_RIGHT_PLYR3")
    {
        println!("Player 3 MOVE_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_RIGHT_PLYR3"));
    }
    if kurinji.is_action_active("MOVE_FORWARD_PLYR3")
    {
        println!("Player 3 MOVE_FORWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_FORWARD_PLYR3"));
    }
    if kurinji.is_action_active("MOVE_BACKWARD_PLYR3")
    {
        println!("Player 3 MOVE_BACKWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_BACKWARD_PLYR3"));
    }
    if kurinji.is_action_active("QUIT_APP_PLYR3")
    {
        println!("Player 3 Quiting...");
        app_exit_events.send(AppExit);
    }
    // PLAYER 4
    if kurinji.is_action_active("BACK_PLYR4")
    {
        println!("Player 4 wants to go back");
    }
    if kurinji.is_action_active("JUMP_PLYR4")
    {
        println!("Player 4 Jumping...");
    }
    if kurinji.is_action_active("SHOOT_PLYR4")
    {
        println!("Player 4 Bang");
    }
    if kurinji.is_action_active("AIM_UP_PLYR4")
    {
        println!("Player 4 AIM_UP... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_UP_PLYR4"));
    }
    if kurinji.is_action_active("AIM_DOWN_PLYR4")
    {
        println!("Player 4 AIM_DOWN... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_DOWN_PLYR4"));
    }
    if kurinji.is_action_active("AIM_LEFT_PLYR4")
    {
        println!("Player 4 AIM_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_LEFT_PLYR4"));
    }
    if kurinji.is_action_active("AIM_RIGHT_PLYR4")
    {
        println!("Player 4 AIM_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_RIGHT_PLYR4"));
    }
    if kurinji.is_action_active("MOVE_LEFT_PLYR4")
    {
        println!("Player 4 MOVE_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_LEFT_PLYR4"));
    }
    if kurinji.is_action_active("MOVE_RIGHT_PLYR4")
    {
        println!("Player 4 MOVE_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_RIGHT_PLYR4"));
    }
    if kurinji.is_action_active("MOVE_FORWARD_PLYR4")
    {
        println!("Player 4 MOVE_FORWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_FORWARD_PLYR4"));
    }
    if kurinji.is_action_active("MOVE_BACKWARD_PLYR4")
    {
        println!("Player 4 MOVE_BACKWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_BACKWARD_PLYR4"));
    }
    if kurinji.is_action_active("QUIT_APP_PLYR4")
    {
        println!("Player 4 Quiting...");
        app_exit_events.send(AppExit);
    }
}

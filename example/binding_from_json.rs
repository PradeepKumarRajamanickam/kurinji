use std::fs;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::app::Events;
use bevy::ecs::ResMut;
use kurinji::{
    Kurinji,
    KurinjiPlugin,
};
fn main()
{
    println!("Kurinji Binding From Json Example");
    App::build().add_plugins(DefaultPlugins)
                // setup
                .add_plugin(KurinjiPlugin::default())
                .add_startup_system(setup.system())
                .add_system(action_system.system())
                .run();
}
fn setup(mut kurinji: ResMut<Kurinji>)
{
    let binding_json = fs::read_to_string("example/config/binding.json")
        .expect("Error! could not open config file");
    kurinji.set_bindings_with_json(&binding_json);
}
fn action_system(kurinji: Res<Kurinji>,
                 mut app_exit_events: ResMut<Events<AppExit>>)
{
    if kurinji.is_action_active("BACK_PLAYER1")
    {
        println!("Player 1 wants to go back");
    }
    if kurinji.is_action_active("BACK_PLAYER2")
    {
        println!("Player 2 wants to go back");
    }
    if kurinji.is_action_active("JUMP")
    {
        println!("Jumping...");
    }
    if kurinji.is_action_active("SHOOT")
    {
        println!("Bang");
    }
    if kurinji.is_action_active("AIM_UP")
    {
        println!("AIM_UP... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_UP"));
    }
    if kurinji.is_action_active("AIM_DOWN")
    {
        println!("AIM_DOWN... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_DOWN"));
    }
    if kurinji.is_action_active("AIM_LEFT")
    {
        println!("AIM_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_LEFT"));
    }
    if kurinji.is_action_active("AIM_RIGHT")
    {
        println!("AIM_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("AIM_RIGHT"));
    }
    if kurinji.is_action_active("MOVE_LEFT")
    {
        println!("MOVE_LEFT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_LEFT"));
    }
    if kurinji.is_action_active("MOVE_RIGHT")
    {
        println!("MOVE_RIGHT... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_RIGHT"));
    }
    if kurinji.is_action_active("MOVE_FORWARD")
    {
        println!("MOVE_FORWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_FORWARD"));
    }
    if kurinji.is_action_active("MOVE_BACKWARD")
    {
        println!("MOVE_BACKWARD... [ strength: {}] ",
                 kurinji.get_action_strength("MOVE_BACKWARD"));
    }
    if kurinji.is_action_active("QUIT_APP")
    {
        println!("Quiting...");
        app_exit_events.send(AppExit);
    }
}

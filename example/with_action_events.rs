use std::fs;
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::ecs::system::ResMut;
use kurinji::{OnActionActive, OnActionEnd, Kurinji, KurinjiPlugin};
fn main() {
    println!("Kurinji Action Events");
    App::build()
        .add_plugins(DefaultPlugins)
        // setup
        .add_plugin(KurinjiPlugin::default())
        .add_startup_system(setup.system())
        .add_system(action_active_events_system.system())
        .add_system(action_end_events_system.system())
        .run();
}
fn setup(mut kurinji: ResMut<Kurinji>) {
    let binding_json = fs::read_to_string("example/config/binding.json")
        .expect("Error! could not open config file");
    kurinji.set_bindings_with_json(&binding_json);
}
fn action_end_events_system(
    mut reader: EventReader<OnActionEnd>,
    mut writer: EventWriter<AppExit>,
) {
    for event in reader.iter() {
        if event.action == "QUIT_APP" {
            println!("Quitting...");
            writer.send(AppExit);
        }
    }
}
fn action_active_events_system(mut reader: EventReader<OnActionActive>) {
    for event in reader.iter() {
        if event.action == "JUMP" {
            println!("Jumping...");
        }
        if event.action == "SHOOT" {
            println!("Bang");
        }
        if event.action == "AIM_UP" {
            println!("AIM_UP... [ strength: {}] ", event.strength);
        }
        if event.action == "AIM_DOWN" {
            println!("AIM_DOWN... [ strength: {}] ", event.strength);
        }
        if event.action == "AIM_LEFT" {
            println!("AIM_LEFT... [ strength: {}] ", event.strength);
        }
        if event.action == "AIM_RIGHT" {
            println!("AIM_RIGHT... [ strength: {}] ", event.strength);
        }
    }
}

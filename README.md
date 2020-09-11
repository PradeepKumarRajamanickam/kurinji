# Bevy Input Mapper
Input Mapper decouples gameplay code from device specific input api. By converting user inputs from different input hardware into game specific actions, eg. *keyboard "Space" or joystick "A" can be mapped to "Jump" Action*. This improves the overall code quality, by keeping the gameplay code separate from input code.
crate: https://crates.io/crates/bevy_prototype_input_map

## Features
- new* Support for custom strength curve function
- Keyboard Key Mapping
- Mouse Button Mapping
- Mouse Axis Mapping
- Action Strength
- Action Deadzone

## Usage
```rust
fn main() {
    App::build()
        ...
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        .add_system(system.system())
        .run();
}
fn setup(
    mut key_map: ResMut<KeyboardMap>
) {

    key_map.bind_keyboard_pressed(KeyCode::Space, "JUMP".to_string());
}
fn system(input_map: Res<InputMap>) {
    if input_map.is_action_in_progress("JUMP".to_string()) {
        println!("Jumping...");
    }
```

*Check out [example/input_map.rs](https://github.com/PradeepKumarRajamanickam/bevy_input_map/blob/master/example/input_map.rs)*

## Example
Use command
> cargo run --example input_map

## Planned
Joystick Mapping
> Depends on bevy input support for joystick

RON based binding config file
>Allow bindings to be loaded from a ron string/file

Context based binding switch
>Allow multiple binding sets to be defined in the ron config. That can be swapped out based on the context, eg.
"E" can be mapped to "use" action in game view but
can be remapped to "equip" action in game inventory ui as shortcut...etc. Based on the view context the bindings will be swapped.

## Author
PradeepKumar Rajamanickam

## Acknowledgments
Inspired by 
- Godot Input Mapper
[https://godotengine.org/article/handling-axis-godot]
- Unreal Action/Axis Mapping
  [https://docs.unrealengine.com/en-US/Gameplay/Input/index.html]

# Bevy Input Map
Input Map decouples gameplay code from device specific input api. By converting user inputs from different input hardware into game specific actions, eg. *keyboard "Space" or joystick "A" can be mapped to "Jump" Action*. This improves the overall code quality, by keeping the gameplay code separate from input code.

## Usage

*Add to Cargo.toml dependencies*
```
[dependencies]
bevy_prototype_input_map = "0.2"
```

*In code*
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
    mut input_map: ResMut<InputMap>,
) {
    input_map
    .bind_keyboard_pressed(KeyCode::Return, "SHOOT")
    .bind_mouse_motion(Axis::YNegative, "AIM_UP")
    .set_dead_zone("AIM_UP", 0.1)
}

// system
fn system(input_map: Res<InputMap>) {
    if input_map.is_action_active("SHOOT") {
        println!("Bang...");
    }
```

*Check out [examples](https://github.com/PradeepKumarRajamanickam/bevy_input_map/tree/master/example)

## Example
Use commands

Via Code
> cargo run --example with_code

Via JSON
> cargo run --example with_json

For Action Events Usage
> cargo run --example with_action_events

## Features
- new* Joystick Support: Button & Analog Input
- new* Event Phase: Ability to set at which event phase an action is active
- new* Action Events: OnActionBegin, OnActionProgress, OnActionEnd
- Binding Stack: Ability to Push, Additive Push and Pop bindings
- JSON/RON Support: Ability to use serialised string to setup bindings
- Support to set custom strength curve function
- Keyboard Key Mapping
- Mouse Button Mapping
- Mouse Move Mapping
- Action Strength
- Action Deadzone

## Repo
https://github.com/PradeepKumarRajamanickam/bevy_input_map/

Note* Latest commit on master branch might be unstable. Use the release tags if you are looking for stable commits or grab
crate from [crate.io](https://crates.io/crates/bevy_prototype_input_map)

## Bug Report
https://github.com/PradeepKumarRajamanickam/bevy_input_map/issues

# Release Notes
## v0.1.3 (18 Sept, 2020)
- Binding Stack
- JSON & RON Support
  
## v0.1.2 (14 Sept, 2020)
- New API
- Ability to set custom strength curve

## v0.1.1 (7 Sept, 2020)
- minor* Readme changes
  - Had to bump the version to publish some readme changes

## v0.1.0 (7 Sept, 2020)
- Keyboard Key Mapping
  - Key press can now be binded to action
- Mouse Button Mapping
  - Mouse button press can now be binded to action
- Mouse Move Mapping
  - Mouse move event can now be mapped to action
- Action Strength
  - Can now query strength of an action. 
  - It will be in range of 0.0 - 1.0
  - Useful for analog inputs like joystick
- Action Deadzone
  - For analog inputs sometimes it is meaningful to have a min threshold to avoid small input noise and to reduce sensitivity

## Author
Pradeep Kumar Rajamanickam

## Acknowledgments
Inspired by 
- Godot Input Mapper
[https://godotengine.org/article/handling-axis-godot]
- Unreal Action/Axis Mapping
  [https://docs.unrealengine.com/en-US/Gameplay/Input/index.html]

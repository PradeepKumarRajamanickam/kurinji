# Kurinji
![kurinji](https://github.com/PradeepKumarRajamanickam/kurinji/blob/master/img_kurinji.png?raw=true)

Input map plugin for bevy. It decouples gameplay code from device specific input api. Converts user inputs from different input hardware into game specific actions, eg. *keyboard "Space" or joystick "A" can be mapped to "Jump" Action*.

## Usage

*Add to Cargo.toml dependencies*
```
[dependencies]
kurinji = "0.1"
```

```rust
fn main() {
    App::build()
        .add_plugin(KurinjiPlugin::default())
        .add_startup_system(setup.system())
        .add_system(system.system())
        .run();
}

fn setup(
    mut kurinji: ResMut<Kurinji>,
) {
    // with data 
    // refer "example/config/gamepad.ron"
    let binding_ron = fs::read_to_string("example/config/gamepad.ron").unwrap()
    kurinji.set_bindings_with_ron(&binding_ron);

    // or

    // via code
    kurinji
    .bind_keyboard_pressed(KeyCode::Return, "SHOOT")
    .bind_mouse_motion(Axis::YNegative, "AIM_UP")
    .set_dead_zone("AIM_UP", 0.1)
}

// system
fn system(kurinji: Res<Kurinji>) {
    if input_map.is_action_active("SHOOT") {
        println!("Bang...");
    }
```

*Check out [examples](https://github.com/PradeepKumarRajamanickam/kurinji/tree/master/example)

## Example
Use commands

Via Code
> cargo run --example keyboard_mouse_with_code

> cargo run --example gamepad_with_code

Via JSON/RON
> cargo run --example keyboard_mouse_with_json

> cargo run --example gamepad_with_ron

For Action Events Usage
> cargo run --example with_action_events

## Features
- Mouse Mapping
- Keyboard Mapping
- Joystick Mapping
- Action Strength
- Action Deadzone
- Custom Strength Curve
- JSON/RON Support: Ability to use serialised string to setup bindings
- Binding Stack: Ability to Push, Additive Push and Pop bindings
- Event Phase: Ability to set at which event phase an action is active
- Action Events: OnActionBegin, OnActionProgress, OnActionEnd

## Bug Report
https://github.com/PradeepKumarRajamanickam/kurinji/issues

# Release Notes
## v1.0.0 (21 Nov, 2020)
- rebranded* as Kurinji
  
## v0.1.5 (18 Nov, 2020)
- Joystick Support
- Improved Documentations
  
## v0.1.4 (03 Oct, 2020)
- Event Phase
- Action Events

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

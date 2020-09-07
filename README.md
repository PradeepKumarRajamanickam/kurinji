# Bevy Input Mapper
Input Map will convert user inputs from different input hardware into game specific actions, eg. *keyboard "Space" or joystick "A" can be mapped to "Jump" Action*. This action is consumed ingame. This allows decoupling of the game code from device specific input api.

## Usage
```rust
fn main() {
    App::build()
        ...
        .add_plugin(InputMapPlugin::default())
        .add_startup_system(setup.system())
        ...
        .run();
}
fn setup(
    mut key_map: ResMut<KeyboardMap>
) {

    key_map.bind_keyboard_pressed(KeyCode::Space, "JUMP".to_string());
    ...
}

```

*Check out [example/input_map.rs](https://github.com/PradeepKumarRajamanickam/bevy_input_map/blob/master/example/input_map.rs)*

## Example
Use command
> cargo run --example input_map

## Features
- Keyboard Key Mapping
- Mouse Button Mapping
- Mouse Axis Mapping
- Action Strength
- Action Deadzone

## TBD
Custom Strength Curves
> By default it is linear. User can assign custom curve function if needed
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

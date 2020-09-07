# Bevy Input Mapper
Input Mapper will convert user input from different input hardware into game specific actions.

Like
>*Keyboard "Space" or Joystick "A" can be mapped "Jump" Action.*

This allows game code to not be tightly coupled with a particular device event.

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

> Check out [example/input_map.rs](https://github.com/PradeepKumarRajamanickam/bevy_input_map/blob/master/example/input_map.rs)

## Features
- Keyboard Key Mapping
- Mouse Button Mapping
- Mouse Axis Mapping
- Action Strength
- Action Deadzone

## TBD
### *Joystick Mapping
> Depends on bevy input support for joystick
### *RON based binding config file
>Allow bindings to be loaded from a ron string/file
### *Context based binding switch
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

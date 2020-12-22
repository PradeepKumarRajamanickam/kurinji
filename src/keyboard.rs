use crate::kurinji::Kurinji;
use bevy::prelude::KeyCode;
use bevy::ecs::{
    Res,
    ResMut,
};
use bevy::input::Input;
impl Kurinji
{
    // publics
    pub fn bind_keyboard_pressed(&mut self,
                                 code: KeyCode,
                                 action: &str)
                                 -> &mut Kurinji
    {
        self.keyboard_action_binding
            .insert(code, action.to_string());
        self
    }

    pub fn unbind_keyboard_pressed(&mut self, code: KeyCode) -> &mut Kurinji
    {
        self.keyboard_action_binding.remove(&code);
        self
    }

    // crates
    // system
    pub(crate) fn kb_key_press_input_system(mut input_map: ResMut<Kurinji>,
                                            key_input: Res<Input<KeyCode>>)
    {
        // let map = &mut input_map;
        let bindings_iter = input_map.keyboard_action_binding.clone();
        for (keycode, action) in bindings_iter.iter()
        {
            if key_input.pressed(*keycode)
            {
                input_map.set_raw_action_strength(action, 1.0);
            }
        }
    }
}

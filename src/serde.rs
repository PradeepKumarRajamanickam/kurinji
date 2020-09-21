use crate::{bindings::Bindings, inputmap::InputMap};


impl  InputMap {
    // ron
    pub fn get_bindings_as_ron(&self) -> Result<String, String> {
        let data = self.get_bindings();
        let pretty = ron::ser::PrettyConfig::new()
            .with_enumerate_arrays(true)
            .with_new_line("\n".to_string());
        let serialized = ron::ser::to_string_pretty(&data, pretty);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err("Failed to generate ron".to_string()),
        }
    }
    pub fn set_bindings_with_ron(&mut self, ron: &str) {
        let bindings: Bindings = InputMap::get_bindings_from_ron(ron);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
    }

    // json
    pub fn get_bindings_as_json(&self) -> Result<String, String> {
        let data = self.get_bindings();
        let serialized = serde_json::to_string_pretty(&data);
        match serialized {
            Ok(s) => Ok(s),
            Err(e) => Err("Failed to generate json".to_string()),
        }
    }
    pub fn set_bindings_with_json(&mut self, json: &str) {
        let bindings: Bindings = InputMap::get_bindings_from_json(json);
        self.set_bindings(bindings);

        self.action_strength_curve.clear();
    }
}
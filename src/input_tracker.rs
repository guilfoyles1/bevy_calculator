use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct ButtonState {
    pub inputs: Vec<String>, // Store button inputs as a list of strings
}

impl ButtonState {
    // Convert the input vector into a concatenated string
    pub fn get_current_input(&self) -> String {
        self.inputs.concat() // Merge all elements into one string
    }

    // Clear the stored inputs
    pub fn clear_input(&mut self) {
        self.inputs.clear();
    }

    // Add new input to the state
    pub fn add_input(&mut self, value: String) {
        self.inputs.push(value);
    }
}

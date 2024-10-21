use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct ButtonState {
    pub buttons: Vec<String>, // Store button buttons as a list of strings
}

impl ButtonState {
    // Convert the input vector into a single concatenated string
    pub fn concat_buttons(&self) -> String {
        self.buttons.join("") // Join all elements into one string
    }
}

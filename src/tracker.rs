use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct input_tracker {
    // Vector to hold the sequence of button labels clicked
    pub entries: Vec<String>,
}

impl input_tracker {
    // Method to concatenate the entries into a single string
    pub fn as_concatenated_string(&self) -> String {
        self.entries.join("") // Join all entries into one continuous string
    }
}

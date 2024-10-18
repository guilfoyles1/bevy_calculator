mod component;
mod theme; // Module for UI colors and styles
mod evaluation; // Module for sequence evaluation logic
mod button_handling; // Module for managing button interactions
mod input_tracker; // Module for tracking button clicks and inputs
mod ui_setup; // Module for setting up the UI layout and elements

use bevy::prelude::*;
use bevy::winit::WinitSettings;
use button_handling::button_system; // System to handle button interactions
use ui_setup::initialize_ui; // Function to set up the UI at startup

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Add default Bevy plugins for the application
        .insert_resource(WinitSettings::desktop_app()) // Configure the app as a desktop application
        .insert_resource(input_tracker::InputTracker::default()) // Initialize the input tracker for button presses
        .add_systems(Startup, initialize_ui) // Set up the UI elements when the app starts
        .add_systems(Update, button_system) // Update the button system each frame
        .run(); // Start the Bevy application
}

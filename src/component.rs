use bevy::prelude::Component;

// Represents the label of a button in the UI
#[derive(Component)]
pub struct ActionText(pub String);

// Marker component for an interactive bubble element
#[derive(Component)]
pub struct InteractiveBubble;

// Marker component for displaying the current sequence of inputs
#[derive(Component)]
pub struct InputDisplay;

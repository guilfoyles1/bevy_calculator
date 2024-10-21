use bevy::prelude::Component;

// Represents the label of a button in the UI
#[derive(Component)]
pub struct ButtonName(pub String);

// Marker component for an interactive bubble element
#[derive(Component)]
pub struct InteractiveBubble;

// Marker component for displaying the current sequence of buttons
#[derive(Component)]
pub struct InputDisplay;

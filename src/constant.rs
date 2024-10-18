use bevy::prelude::Color;

// Button color definitions for different interaction states
pub const DEFAULT_BUTTON: Color = Color::rgb(0.2, 0.6, 0.8); // Sky blue for idle state
pub const ACTIVE_HOVER_BUTTON: Color = Color::rgb(0.3, 0.8, 1.0); // Light blue when hovered
pub const CLICKED_BUTTON: Color = Color::rgb(0.1, 0.4, 0.6); // Deep blue when pressed

// UI accent color for additional elements like highlights or borders
pub const HIGHLIGHT_COLOR: Color = Color::rgb(0.9, 0.3, 0.4); // Reddish-pink accent

// General background and border colors for the interface
pub const MAIN_BACKGROUND: Color = Color::rgb(0.95, 0.95, 0.85); // Soft beige for the background
pub const OUTLINE_COLOR: Color = Color::rgb(0.0, 0.5, 0.5); // Teal for borders and outlines

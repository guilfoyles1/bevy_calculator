use bevy::prelude::Color;

// Button color definitions for different interaction states
pub const DEFAULT_BUTTON: Color = Color::srgb(0.2, 0.6, 0.8); // Sky blue for idle state
pub const HOVERED_BUTTON: Color = Color::srgb(0.3, 0.8, 1.0); // Light blue when hovered
pub const PRESSED_BUTTON: Color = Color::srgb(0.1, 0.4, 0.6); // Deep blue when pressed

// General background and border colors for the interface
pub const MAIN_BACKGROUND: Color = Color::srgb(0.95, 0.95, 0.85); // Soft beige for the background
pub const BORDER_COLOR: Color = Color::srgb(0.0, 0.5, 0.5); // Teal for borders and outlines

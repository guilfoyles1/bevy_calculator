use bevy::prelude::*;
use crate::component::{ButtonName, InteractiveBubble, InputDisplay};
use crate::theme::{DEFAULT_BUTTON, MAIN_BACKGROUND, BORDER_COLOR}; // Import UI theme colors

pub fn initialize_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera for the UI
    commands.spawn(Camera2dBundle::default());

    // Create the root node for the UI layout
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: MAIN_BACKGROUND.into(), // Set the main background color
            ..default()
        })
        .with_children(|parent| {
            // Spawn the interactive bubble for input display
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Px(50.0),
                        margin: UiRect {
                            top: Val::Px(20.0), // Margin from the top
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.5, 0.9).into(), // Light blue for the bubble
                    ..default()
                },
                InteractiveBubble, // Marker component for interactive behavior
            ))
            .with_children(|parent| {
                // Add the text section for displaying the input sequence
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(), // Initialize with empty value
                            style: TextStyle {
                                font: asset_server.load("/Users/shaynaguilfoyle/bevy_calculator-1/Font/Rows_of_Sunflowers.ttf"), // Load font
                                font_size: 32.0,
                                color: Color::WHITE, // Text color
                            },
                        }],
                        ..default()
                    },
                    ..default()
                })
                .insert(InputDisplay); // Marker for displaying the input sequence
            });

            // Define the button layout in rows
            let button_grid: Vec<Vec<&str>> = vec![
                vec!["7", "8", "9", "/"],
                vec!["4", "5", "6", "*"],
                vec!["1", "2", "3", "-"],
                vec!["C", "0", "+/-", "+"],
                vec![".", "="],
            ];

            // Create button rows
            for row in button_grid {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween, // Space between buttons
                            margin: UiRect {
                                bottom: Val::Px(10.0), // Bottom margin between rows
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Create buttons in the current row
                        for label in row {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0), // Set button width
                                            height: Val::Px(65.0), // Set button height
                                            margin: UiRect {
                                                left: Val::Px(5.0), // Left margin
                                                right: Val::Px(5.0), // Right margin
                                                top: Val::Px(10.0), // Top margin
                                                bottom: Val::Px(10.0), // Bottom margin
                                            },
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(5.0)), // Set border size
                                            ..default()
                                        },
                                        background_color: DEFAULT_BUTTON.into(), // Set button background color
                                        border_color: BorderColor(BORDER_COLOR), // Set button outline color
                                        ..default()
                                    },
                                    ButtonName(label.to_string()), // Create button label
                                ))
                                .with_children(|parent| {
                                    // Set the text for the button
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            label.to_string(),
                                            TextStyle {
                                                font: asset_server.load("/Users/shaynaguilfoyle/bevy_calculator-1/Font/Rows_of_Sunflowers.ttf"), // Load custom font
                                                font_size: 32.0,
                                                color: Color::BLACK, // Set button text color
                                            },
                                        ),
                                        ..default()
                                    });
                                });
                        }
                    });
            }
        });
}

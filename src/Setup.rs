use bevy::prelude::*;
use crate::component::{ActionText, InteractiveBubble, InputDisplay};
use crate::theme::{DEFAULT_BUTTON, MAIN_BACKGROUND, OUTLINE_COLOR}; // Changed constants module name to 'theme'

pub fn initialize_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Root node for the entire UI layout
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
            background_color: MAIN_BACKGROUND.into(), // Set background to a soft beige color
            ..default()
        })
        .with_children(|parent| {
            // Create the display area for user input (Bubble)
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(60.0),
                        height: Val::Px(50.0),
                        margin: UiRect {
                            top: Val::Px(20.0),
                            ..default()
                        },
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.5, 0.9).into(), // Light blue for the display
                    ..default()
                },
                InteractiveBubble, // Marker component for interactive bubble
            ))
            .with_children(|parent| {
                // Add the text display for the input sequence
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 32.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..default()
                    },
                    ..default()
                })
                .insert(InputDisplay); // Marker for sequence display component
            });

            // Define the layout for the calculator buttons in rows
            let button_grid: Vec<Vec<&str>> = vec![
                vec!["7", "8", "9", "/"],
                vec!["4", "5", "6", "*"],
                vec!["1", "2", "3", "-"],
                vec!["C", "0", "+/-", "+"],
                vec![".", "="],
            ];

            // Iterate through each row of buttons and spawn them
            for row in button_grid {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween, // Add space between buttons in rows
                            margin: UiRect {
                                bottom: Val::Px(10.0), // Spacing between rows
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Create each button in the row
                        for label in row {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Px(150.0),
                                            height: Val::Px(65.0),
                                            margin: UiRect {
                                                left: Val::Px(5.0), // Horizontal spacing between buttons
                                                right: Val::Px(5.0),
                                                top: Val::Px(10.0),
                                                bottom: Val::Px(10.0),
                                            },
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(5.0)), // Set border thickness
                                            ..default()
                                        },
                                        background_color: DEFAULT_BUTTON.into(),
                                        border_color: BorderColor(OUTLINE_COLOR), // Teal outline for buttons
                                        ..default()
                                    },
                                    ActionText(label.to_string()), // Assign label text to button
                                ))
                                .with_children(|parent| {
                                    // Set the button text with custom font and style
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            label.to_string(),
                                            TextStyle {
                                                font: asset_server.load("fonts/Rows_of_Sunflowers.ttf"), // Custom font
                                                font_size: 32.0,
                                                color: Color::BLACK,
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

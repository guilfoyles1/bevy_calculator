use bevy::prelude::*;
use crate::component::{ButtonName, InputDisplay};
use crate::theme::{DEFAULT_BUTTON, HOVERED_BUTTON, PRESSED_BUTTON, BORDER_COLOR}; // Updated module name to 'theme'
use crate::input_tracker::ButtonState; // Updated module name to 'input_tracker'
use crate::calculation::{evaluate_sequence, toggle_last_number_sign}; // Updated module name to 'calculation'

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &ButtonName,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text, With<InputDisplay>>,
    mut input_tracker: ResMut<ButtonState>,
) {
    for (interaction, mut color, mut border_color, button_label) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into(); // Darker color on press
                border_color.0 = BORDER_COLOR; // Maintain border color on press

                match button_label.0.as_str() {
                    "=" => {
                        let result = evaluate_sequence(&input_tracker.concat_buttons());
                        for mut text in text_query.iter_mut() {
                            text.sections[0].value = format!("{}", result);
                        }
                        input_tracker.buttons.clear();
                        input_tracker.buttons.push(result.clone());
                    }
                    "C" => {
                        input_tracker.buttons.clear();
                        for mut text in text_query.iter_mut() {
                            text.sections[0].value = "".to_string();
                        }
                    }
                    "+/-" => toggle_last_number_sign(&mut input_tracker),
                    _ => input_tracker.buttons.push(button_label.0.clone()),
                }

                for mut text in text_query.iter_mut() {
                    text.sections[0].value = input_tracker.concat_buttons();
                }

                println!(
                    "Button {} clicked! Current sequence: {}",
                    button_label.0, input_tracker.concat_buttons()
                );
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into(); // Lighter color on hover
                border_color.0 = Color::WHITE; // Change border color on hover
            }
            Interaction::None => {
                *color = DEFAULT_BUTTON.into(); // Default button color
                border_color.0 = BORDER_COLOR; // Default border color
            }
        }
    }
}

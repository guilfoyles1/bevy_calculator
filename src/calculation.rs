use std::collections::VecDeque;
use crate::input_tracker::ClickedButtons; // Changed module name to 'input_tracker'

/// Evaluates a mathematical sequence given as a string and returns the result as a string.
pub fn evaluate_sequence(sequence: &str) -> String {
    let mut numbers: VecDeque<f64> = VecDeque::new(); // Queue for numbers
    let mut operators: VecDeque<char> = VecDeque::new(); // Queue for operators
    let mut current_number = String::new(); // Buffer for the current number

    // Applies the given operator to the two topmost numbers in the queue
    fn apply_operator(numbers: &mut VecDeque<f64>, operator: char) {
        let b = numbers.pop_back().unwrap(); // Second operand
        let a = numbers.pop_back().unwrap(); // First operand
        let result = match operator {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => {
                if b == 0.0 {
                    println!("Error: Division by zero"); // Handle division by zero
                    return; 
                }
                a / b
            }
            _ => {
                println!("Error: Invalid operator"); // Handle invalid operator
                return; 
            }
        };
        numbers.push_back(result); // Push the result back to the numbers queue
    }

    let mut prev_char: Option<char> = None; // Store previous character

    // Iterate through each character in the sequence
    for c in sequence.chars() {
        if c.is_digit(10) || c == '.' { // Handle digits and decimals
            current_number.push(c);
        } else if c == '-' && (prev_char.is_none() || "+-*/(".contains(prev_char.unwrap())) {
            // Handle negation as a standalone number
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push_back(num);
                    current_number.clear();
                }
            }
            current_number.push(c); // Add the '-' sign
        } else if "+-*/()".contains(c) { // Handle operators and parentheses
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<f64>() {
                    numbers.push_back(num);
                    current_number.clear();
                } else {
                    return "Invalid input".to_string(); // Handle parsing error
                }
            }

            if c == '(' {
                operators.push_back(c); // Push opening parenthesis
            } else if c == ')' {
                while let Some(&top_operator) = operators.back() {
                    if top_operator == '(' {
                        operators.pop_back(); // Pop opening parenthesis
                        break;
                    }
                    apply_operator(&mut numbers, operators.pop_back().unwrap()); // Apply operators until the opening parenthesis
                }
            } else {
                while let Some(&top_operator) = operators.back() {
                    if precedence(top_operator) >= precedence(c) {
                        apply_operator(&mut numbers, operators.pop_back().unwrap()); // Apply operator based on precedence
                    } else {
                        break;
                    }
                }
                operators.push_back(c); // Push the current operator
            }
        }
        prev_char = Some(c); // Update previous character
    }

    // Parse any remaining number in the buffer
    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<f64>() {
            numbers.push_back(num);
        } else {
            return "Invalid input".to_string(); // Handle parsing error
        }
    }

    // Apply remaining operators
    while let Some(operator) = operators.pop_back() {
        apply_operator(&mut numbers, operator);
    }

    // Format the final result
    let final_result = numbers.pop_back().map_or("Invalid input".to_string(), |result| {
        format!("{:.4}", result) // Round to four decimal places
    });

    final_result
}

/// Toggles the sign of the last number in the ClickedButtons struct.
pub fn toggle_last_number_sign(input_tracker: &mut ClickedButtons) {
    if let Some(last) = input_tracker.buttons.last_mut() {
        if let Ok(mut number) = last.parse::<f64>() {
            number = -number; // Negate the number
            *last = number.to_string(); // Update the last entry
        } else if last == "+" {
            *last = "-".to_string(); // Toggle between '+' and '-' if the last entry is a sign
        } else if last == "-" {
            *last = "+".to_string();
        }
    }
}

/// Determines the precedence of operators for evaluation.
pub fn precedence(operator: char) -> usize {
    match operator {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

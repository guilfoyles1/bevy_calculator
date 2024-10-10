# Bevy Calculator Project

This project is a basic calculator application built using the **Bevy** game engine. It showcases how Bevy can be used to create interactive user interfaces with support for standard arithmetic operations like addition, subtraction, multiplication, and division.

---

## Key Features

- **Interactive Button Layout**: Clickable buttons for digits (0-9) and operators (+, -, *, /, =), each with immediate visual feedback.
- **Simple Arithmetic Functions**: The calculator performs basic calculations.
- **UI Interactivity**: A minimal UI setup with responsive design for button clicks.

---

## Setup Instructions

### Prerequisites:

Before running the project, ensure you have these installed:

1. **Rust Programming Language**: If it's not installed yet, follow [these instructions](https://www.rust-lang.org/tools/install) to set it up.
2. **Bevy Game Engine**: A Rust-based ECS (Entity-Component-System) framework.

### Installation:

To get started, follow these steps:

1. **Clone the repository**:

    ```sh
    git clone https://github.com/
    ```

2. **Enter the directory**:

    ```sh
    cd BevyCalculator/calculator
    ```

3. **Run the project**:

    ```sh
    cargo run
    ```

---

## Directory Overview

The project is structured as follows:

```
bevy-calculator/
│
├── src/
│   ├── main.rs          # Main application logic.
│   ├── components.rs    # Custom components for UI buttons.
│   ├── systems.rs       # Button interaction logic.
│   └── styles.rs        # Style and layout configuration for the UI.
│
├── assets/
│   └── fonts/           # Fonts used in the UI (FiraSans-Bold.ttf).
│
├── Cargo.toml           # Contains dependencies, including Bevy.
└── README.md            # You're reading it!
```

---

## How to Use

- Launch the calculator using the commands above.
- Enter numbers and perform operations by clicking the on-screen buttons.
- Press the `=` button to calculate and display the result.

---

## Current Limitations

- **Basic Functionality**: The calculator currently supports only basic arithmetic. Advanced operations (such as parentheses or exponents) are not included.
- **UI Alignment**: If the buttons are not aligned properly, check the alignment settings in the `styles.rs` file.

---

## License

This project is licensed under the **MIT License**. See the LICENSE file for more information.

---

## Special Mentions

- **Bevy**: A great game engine built for Rust.
- **Rust**: For making low-level programming easier and safer.

extern crate piston_window;

mod sorting_algorithms;
use sorting_algorithms::bubble_sort::bubble_sort;

mod state_logic;
use state_logic::state_bs::bubble_sort_visualization;
use state_logic::state_choice::choice_window;
use state_logic::state_mm::main_menu;

mod buttons;
use buttons::but_back::draw_back_button;

use piston_window::*;

use std::time::{Duration, Instant};

enum AppState {
    MainMenu,
    ChoiceWindow,
    BubbleSortVisualization,
}

fn main() {
    let mut arr_original = vec![64, 34, 25, 12, 22, 11, 90]; // Original unsorted array
    let mut arr = arr_original.clone(); // Working array for sorting
    println!("Unsorted array: {:?}", arr);

    let mut state = AppState::MainMenu;
    let mut window: PistonWindow = WindowSettings::new("Sorting Visualization", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut current_step = 0;
    let mut total_steps = 0;
    let mut finished = false;

    // Initialize mouse position
    let mut mouse_pos: [f64; 2] = [0.0, 0.0]; // Store the mouse position globally

    // Step visualization variables for bubble sort
    let mut steps: Vec<Vec<i32>> = vec![];
    let update_delay = Duration::from_millis(500); // Delay of 500 ms between each step
    let mut last_update = Instant::now();

    while let Some(event) = window.next() {

        // Capture mouse position when it moves
        if let Some(pos) = event.mouse_cursor_args() {
            mouse_pos = pos; // Update mouse_pos whenever the mouse moves
        }

        // Handle mouse clicks
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            println!("Mouse clicked at: {:?}", mouse_pos); // Use the stored mouse position

            match state {
                AppState::MainMenu => {
                    // Check if the "Continue" button is clicked in the Main Menu
                    if mouse_pos[0] > 300.0 && mouse_pos[0] < 500.0 && mouse_pos[1] > 300.0 && mouse_pos[1] < 380.0 {
                        println!("Continue button clicked!");
                        state = AppState::ChoiceWindow; // Go to the white screen with the red button
                    }
                }

                AppState::ChoiceWindow => {
                    // Check if the red button is clicked on the white screen
                    if mouse_pos[0] > 300.0 && mouse_pos[0] < 500.0 && mouse_pos[1] > 300.0 && mouse_pos[1] < 380.0 {
                        println!("Red button clicked! Transitioning to bubble sort visualization...");
                        arr = arr_original.clone(); // Reset the array to the original unsorted state
                        steps = bubble_sort(&mut arr); // Run bubble sort and store steps
                        total_steps = steps.len();
                        current_step = 0;
                        last_update = Instant::now(); // Reset the timer for step delay
                        state = AppState::BubbleSortVisualization; // Go to bubble sort visualization
                    }
                }

                AppState::BubbleSortVisualization => {
                    // No specific actions for mouse clicks in this state
                }
            }

            // "Back" button (top-left corner) click handling
            if mouse_pos[0] > 10.0 && mouse_pos[0] < 60.0 && mouse_pos[1] > 10.0 && mouse_pos[1] < 40.0 {
                println!("Back button clicked!");
                match state {
                    AppState::ChoiceWindow => {
                        state = AppState::MainMenu; // Go back to Main Menu
                    }
                    AppState::BubbleSortVisualization => {
                        state = AppState::ChoiceWindow; // Go back to white screen with red button
                    }
                    _ => {} // No back action from MainMenu
                }
            }
        }

        window.draw_2d(&event, |c, g, _| {
            match state {
                // Main Menu
                AppState::MainMenu => {
                    state_logic::state_mm::main_menu(c, g, mouse_pos);
                }

                // White screen with choice (red button)
                AppState::ChoiceWindow => {
                    state_logic::state_choice::choice_window(c, g, mouse_pos);
                }

                // Bubble Sort Visualization
                AppState::BubbleSortVisualization => {
                    if last_update.elapsed() >= update_delay {
                        last_update = Instant::now();
                        current_step += 1;
                        if current_step >= total_steps {
                            current_step = total_steps - 1;
                            finished = true;
                        }
                    }
                    state_logic::state_bs::bubble_sort_visualization(c, g, current_step, &steps, &arr, finished);
                }
            }

            // Draw the back button for every screen
            buttons::but_back::draw_back_button(c, g);
        });

    }
}

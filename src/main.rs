extern crate piston_window;

mod sorting_algorithms;

use piston_window::*;
use sorting_algorithms::bubble_sort::bubble_sort;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", arr);

    let mut window: PistonWindow = WindowSettings::new("Sorting Visualization", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut current_step = 0;
    let mut total_steps = 0;
    let mut finished = false;

    // Introduce a delay system
    let mut last_update = Instant::now();
    let update_delay = Duration::from_millis(500); // Delay of 500 ms between each step

    // Find the max value for the dynamic y-axis
    let max_value = *arr.iter().max().unwrap() as f64;
    let bar_width = (700.0 / arr.len() as f64) - 5.0; // Width of each bar (700 px with spacing)

    let mut is_paused = true; // Flag to indicate a pause before sorting
    let mut steps: Vec<Vec<i32>> = vec![]; // Declare steps but do not fill them yet

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g); // Clear the screen (white background)

            // Display the initial unsorted state or the current sorting step
            let state = if is_paused { &arr } else { &steps[current_step] };

            for (i, &value) in state.iter().enumerate() {
                let bar_height = value as f64 * (500.0 / max_value); // Scale height based on max value

                // Draw the bar with some spacing
                rectangle(
                    [0.0, 0.0, 1.0, 1.0], // Color of the bar (blue)
                    [
                        50.0 + i as f64 * (bar_width + 5.0), // X position with spacing
                        550.0 - bar_height,                  // Y position (inverted)
                        bar_width,                           // Bar width
                        bar_height,                          // Bar height
                    ],
                    c.transform,
                    g,
                );
            }
        });

        if is_paused {
            // If we're in the initial state, we pause for 2 seconds
            if last_update.elapsed() >= Duration::from_secs(2) {
                is_paused = false; // End the pause after 2 seconds
                last_update = Instant::now(); // Reset the timer

                // Now, call the sorting algorithm AFTER the pause, so we capture the unsorted array first
                steps = bubble_sort(&mut arr); // Call bubble_sort and store steps
                total_steps = steps.len(); // Capture the number of steps
            }
        } else if !finished {
            // Sorting visualization starts after the pause
            let now = Instant::now();
            if now.duration_since(last_update) >= update_delay {
                last_update = now; // Reset the timer

                current_step += 1;
                if current_step >= total_steps - 1 {
                    current_step = total_steps - 1;
                    finished = true; // End the sorting process
                }
            }
        }
    }
}

use piston_window::*;

pub fn bubble_sort_visualization(
    c: Context,
    g: &mut G2d,
    current_step: usize,
    steps: &Vec<Vec<i32>>,
    arr: &Vec<i32>,
    finished: bool,
) {
    clear([1.0; 4], g); // Clear the screen (white background)

    // Draw the bars representing the current sorting step
    let state = &steps[current_step];
    let max_value = *arr.iter().max().unwrap() as f64;
    let bar_width = (700.0 / state.len() as f64) - 5.0; // Adjust width with spacing
    for (i, &value) in state.iter().enumerate() {
        let bar_height = value as f64 * (500.0 / max_value); // Scale height based on max value
        rectangle(
            [0.0, 0.0, 1.0, 1.0],
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
}

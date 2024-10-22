use piston_window::*;

// Function to draw the "Back" button
pub fn draw_back_button(c: Context, g: &mut G2d) {
    // Draw "Back" button on every screen
    rectangle([0.8, 0.0, 0.0, 1.0], [10.0, 10.0, 50.0, 30.0], c.transform, g); // Red "Back" button
}
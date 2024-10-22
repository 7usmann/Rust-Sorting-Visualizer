use piston_window::*;

pub fn main_menu(
    c: Context,
    g: &mut G2d,
    mouse_pos: [f64; 2],
) {
    clear([1.0; 4], g); // Clear the screen (white background)

    // Draw a "Title" at the top
    rectangle([0.0, 0.0, 1.0, 1.0], [200.0, 150.0, 400.0, 100.0], c.transform, g);

    // Draw the "Continue" button (as a rectangle)
    rectangle([0.0, 0.5, 0.0, 1.0], [300.0, 300.0, 200.0, 80.0], c.transform, g);
}
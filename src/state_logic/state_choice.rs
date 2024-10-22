use piston_window::*;

pub fn choice_window(
    c: Context,
    g: &mut G2d,
    mouse_pos: [f64; 2],
) {
    clear([1.0; 4], g); // Clear the screen to white

    // Draw a red button
    rectangle([1.0, 0.0, 0.0, 1.0], [300.0, 300.0, 200.0, 80.0], c.transform, g);
}
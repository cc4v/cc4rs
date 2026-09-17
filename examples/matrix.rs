use cc4rs::*;

fn main() {
    run(draw);
}

fn draw() {
    let center_x = width() as f32 * 0.5;
    let center_y = height() as f32 * 0.5;
    let angle = frame_count() as f32 * 0.01;

    push_matrix();
    translate(center_x, center_y, 0.0);
    rotate_z(angle);
    scale(1.2, 1.2, 1.0);
    translate(-50.0, -50.0, 0.0);

    fill();
    set_color(colors::red());
    square(0.0, 0.0, 100.0);
    pop_matrix();
}

use cc4rs::*;

fn main() {
    run(draw);
}

fn draw() {
    let x = mouse_x();
    let y = mouse_y();

    fill();
    set_color(colors::red());
    circle(x, y, 100.0);

    no_fill();
    set_color(colors::blue());
    circle(x + 20.0, y + 20.0, 100.0);
}
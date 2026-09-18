use cc4rs::*;

fn main() {
    run(draw);
}

fn draw() {
    fill();
    set_color(colors::red());
    rect(20.0, 20.0, 100.0, 100.0);

    no_fill();
    set_color(colors::blue());
    rect(10.0, 10.0, 100.0, 100.0);
}

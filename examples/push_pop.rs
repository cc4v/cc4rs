use cc4rs::*;

fn main() {
    run(draw);
}

fn draw() {
    fill();
    set_color(colors::red());
    rect(20.0, 20.0, 100.0, 100.0);

    push_style();
    no_fill();
    set_color(colors::blue());
    rect(10.0, 10.0, 100.0, 100.0);
    pop_style();

    rect(140.0, 20.0, 100.0, 100.0);
}

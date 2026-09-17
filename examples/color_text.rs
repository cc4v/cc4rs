use cc4rs::*;

fn main() {
    run(draw);
}

fn draw() {
    text_size(10);
    set_color(colors::red());
    text("hello", 10.0, 10.0);

    text_size(20);
    set_color(colors::green());
    text("hello", 10.0, 40.0);

    text_size(30);
    set_color(colors::blue());
    text("hello", 10.0, 80.0);
}

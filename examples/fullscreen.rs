use cc4rs::*;

fn main() {
    run(draw);
}

fn draw() {
    text_size(10);
    set_color(colors::red());
    text("Press T", 10.0, 10.0);
    set_color(colors::black());
    text("to toggle fullscreen", 10.0, 30.0);

    if key_just_pressed(Keycode::T) {
        toggle_fullscreen();
    }
}
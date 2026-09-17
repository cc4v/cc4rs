use cc4rs as cc;

fn main() {
    cc::run(draw);
}

fn draw() {
    cc::text_size(10);
    cc::set_color(cc::colors::red());
    cc::text("Press T", 10.0, 10.0);
    cc::set_color(cc::colors::black());
    cc::text("to toggle fullscreen", 10.0, 30.0);

    if cc::key_just_pressed(cc::Keycode::T) {
        cc::toggle_fullscreen();
    }
}
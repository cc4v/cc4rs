use cc4rs as cc;

fn main() {
    cc::run(draw);
}

fn draw() {
    let x = cc::mouse_x();
    let y = cc::mouse_y();

    cc::fill();
    cc::set_color(cc::colors::red());
    cc::circle(x, y, 100.0);

    cc::no_fill();
    cc::set_color(cc::colors::blue());
    cc::circle(x + 20.0, y + 20.0, 100.0);
}
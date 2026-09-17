use cc4rs as cc;

fn main() {
    cc::run(draw);
}

fn draw() {
    cc::fill();
    cc::set_color(cc::colors::red());
    cc::rect(20.0, 20.0, 100.0, 100.0);

    cc::no_fill();
    cc::set_color(cc::colors::blue());
    cc::rect(10.0, 10.0, 100.0, 100.0);
}
use cc4rs as cc;

fn main() {
    cc::run(draw);
}

fn draw() {
    cc::text_size(10);
    cc::set_color(cc::colors::red());
    cc::text("hello", 10.0, 10.0);

    cc::text_size(20);
    cc::set_color(cc::colors::green());
    cc::text("hello", 10.0, 40.0);

    cc::text_size(30);
    cc::set_color(cc::colors::blue());
    cc::text("hello", 10.0, 80.0);
}

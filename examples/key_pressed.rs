use cc4rs as cc;

fn main() {
    cc::run(draw);
}

fn draw() {
    cc::fill();
    cc::set_color(cc::colors::gray());
    cc::rect(0.0, 0.0, 400.0, 400.0);

    if cc::key_pressed() {
        cc::set_color(cc::colors::red());
        cc::rect(10.0, 10.0, 100.0, 100.0);

        cc::set_color(cc::colors::white());
        let key = format!("{:?}", cc::key());
        cc::text(&key, 30.0, 30.0);
    } else {
        cc::set_color(cc::colors::white());
        cc::rect(10.0, 10.0, 100.0, 100.0);
    }
}

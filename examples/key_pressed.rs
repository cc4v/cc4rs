use cc4rs::*;

fn main() {
    run(draw);
}

fn draw() {
    fill();
    set_color(colors::gray());
    rect(0.0, 0.0, 400.0, 400.0);

    if key_pressed() {
        set_color(colors::red());
        rect(10.0, 10.0, 100.0, 100.0);

        set_color(colors::white());
        let key = format!("{:?}", key());
        text(&key, 30.0, 30.0);
    } else {
        set_color(colors::white());
        rect(10.0, 10.0, 100.0, 100.0);
    }
}

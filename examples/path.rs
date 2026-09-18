use cc4rs::*;
use std::f32::consts::PI;

fn main() {
    run(draw);
}

fn draw() {
    fill();
    set_color(colors::dark_blue());
    begin_shape();
    vertex(115.0, 40.0);
    vertex(132.0, 85.0);
    vertex(180.0, 85.0);
    vertex(142.0, 115.0);
    vertex(157.0, 165.0);
    vertex(115.0, 135.0);
    vertex(73.0, 165.0);
    vertex(88.0, 115.0);
    vertex(50.0, 85.0);
    vertex(98.0, 85.0);
    end_shape(true);

    no_fill();
    set_color(colors::red());
    begin_shape();
    move_to(210.0, 100.0);
    bezier_vertex(250.0, 20.0, 300.0, 180.0, 350.0, 100.0);
    end_shape(false);

    set_color(colors::green());
    curve(210.0, 300.0, 250.0, 220.0, 300.0, 360.0, 350.0, 300.0);

    fill();
    set_color(colors::orange());
    triangle(275.0, 60.0, 350.0, 60.0, 312.0, 140.0);

    set_color(colors::blue());
    arc(315.0, 220.0, 90.0, 70.0, 0.0, PI * 1.5);
}

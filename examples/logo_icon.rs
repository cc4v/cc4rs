use cc4rs::*;
use sokol::gl as sgl;

fn main() {
    run(draw);
}

fn draw() {
    let width = width() as f32;
    let height = height() as f32;

    let a = colors::color_u8_from_rgb(100, 250, 154);
    let b = colors::color_u8_from_rgb(247, 253, 236);
    let c = colors::color_u8_from_rgb(255, 94, 44);
    let d = colors::color_u8_from_rgb(119, 102, 229);

    sgl::begin_quads();
    sgl::v2f_c3b(0.0, 0.0, a.r, a.g, a.b);
    sgl::v2f_c3b(width, 0.0, b.r, b.g, b.b);
    sgl::v2f_c3b(width, height, c.r, c.g, c.b);
    sgl::v2f_c3b(0.0, height, d.r, d.g, d.b);
    sgl::end();

    set_color(colors::white());
    text_size(32);
    text("cc4rs:", 100.0, 100.0);
    text_size(14);
    text("creative coding framework", 30.0, 200.0);
    text("for Rust", 30.0, 230.0);
}

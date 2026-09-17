use crate::{with_current_cc, Color};
use sokol::gl as sgl;

pub fn set_color(color: Color) {
    with_current_cc(|cc| cc.current_style.color = color);
}

pub fn fill() {
    with_current_cc(|cc| cc.current_style.fill = true);
}

pub fn no_fill() {
    with_current_cc(|cc| cc.current_style.fill = false);
}

pub fn rect(x: f32, y: f32, width: f32, height: f32) {
    let Some((color, is_filled)) =
        with_current_cc(|cc| (cc.current_style.color, cc.current_style.fill))
    else {
        return;
    };

    sgl::c4f(color.r, color.g, color.b, color.a);
    if is_filled {
        sgl::begin_quads();
        sgl::v2f(x, y);
        sgl::v2f(x + width, y);
        sgl::v2f(x + width, y + height);
        sgl::v2f(x, y + height);
    } else {
        sgl::begin_line_strip();
        sgl::v2f(x, y);
        sgl::v2f(x + width, y);
        sgl::v2f(x + width, y + height);
        sgl::v2f(x, y + height);
        sgl::v2f(x, y);
    }
    sgl::end();
}
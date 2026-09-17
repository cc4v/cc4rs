use crate::{colors, with_current_cc, TextCfg};
use sokol::{app as sapp, debugtext as sdtx};

pub fn init_frame() {
    let _ = (sapp::widthf(), sapp::heightf());
}

pub fn text_size(size: i32) {
    with_current_cc(|cc| cc.current_style.text_config.size = size);
}

pub fn text(message: &str, x: f32, y: f32) {
    let Some(config) = with_current_cc(|cc| cc.current_style.text_config.clone()) else {
        return;
    };
    draw_text(message, x, y, config);
}

fn draw_text(message: &str, x: f32, y: f32, config: TextCfg) {
    let font_size = config.size.max(1) as f32;
    let color = colors::u8_color(config.color);

    sdtx::font(0);
    sdtx::canvas(sapp::widthf() / (font_size / 8.0), sapp::heightf() / (font_size / 8.0));
    sdtx::origin(0.0, 0.0);
    sdtx::color4b(color.r, color.g, color.b, color.a);
    sdtx::pos(x / font_size, y / font_size);
    sdtx::puts(message);
}
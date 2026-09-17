use crate::types;

type Color = types::color::Color;
type ColorU8 = types::color::ColorU8;

pub fn u8_color(c: Color) -> ColorU8 {
    const _255: f32 = 255.0;

    return ColorU8 {
        r: (c.r * _255) as u8,
        g: (c.g * _255) as u8,
        b: (c.b * _255) as u8,
        a: (c.a * _255) as u8,
    };
}

pub fn f32_color(c: ColorU8) -> Color {
    const _255: f32 = 255.0;

    return Color {
        r: (c.r as f32) / _255,
        g: (c.g as f32) / _255,
        b: (c.b as f32) / _255,
        a: (c.a as f32) / _255,
    };
}

pub fn color_u8_from_hex(hex: i32) -> ColorU8 {
    let r: u8 = ((hex >> 16) & 0xFF) as u8;
    let g: u8 = ((hex >> 8) & 0xFF) as u8;
    let b: u8 = ((hex >> 0) & 0xFF) as u8;
    return ColorU8 {
        r: r,
        g: g,
        b: b,
        a: 255,
    };
}

pub fn color_from_hex(hex: i32) -> Color {
    let col_u8 = color_u8_from_hex(hex);
    return f32_color(col_u8);
}

pub fn color_from_rgb(r: u8, g: u8, b: u8) -> Color {
    return Color {
        r: (r as f32) / 255.0,
        g: (g as f32) / 255.0,
        b: (b as f32) / 255.0,
        a: 255.0,
    };
}

pub fn color_from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    return Color {
        r: (r as f32) / 255.0,
        g: (g as f32) / 255.0,
        b: (b as f32) / 255.0,
        a: (a as f32) / 255.0,
    };
}

pub fn color_u8_from_rgb(r: u8, g: u8, b: u8) -> ColorU8 {
    return ColorU8 {
        r: r,
        g: g,
        b: b,
        a: 255,
    };
}

pub fn color_u8_from_rgba(r: u8, g: u8, b: u8, a: u8) -> ColorU8 {
    return ColorU8 {
        r: r,
        g: g,
        b: b,
        a: a,
    };
}

fn color(r: u8, g: u8, b: u8) -> Color {
    return crate::types::color::color(r, g, b, 255);
}

pub fn black() -> Color {
    color(0, 0, 0)
}
pub fn white() -> Color {
    color(255, 255, 255)
}
pub fn gray() -> Color {
    color(128, 128, 128)
}
pub fn red() -> Color {
    color(255, 0, 0)
}
pub fn green() -> Color {
    color(0, 255, 0)
}
pub fn blue() -> Color {
    color(0, 0, 255)
}

pub fn yellow() -> Color {
    color(255, 255, 0)
}
pub fn magenta() -> Color {
    color(255, 0, 255)
}
pub fn cyan() -> Color {
    color(0, 255, 255)
}
pub fn orange() -> Color {
    color(255, 165, 0)
}
pub fn purple() -> Color {
    color(128, 0, 128)
}
pub fn indigo() -> Color {
    color(75, 0, 130)
}
pub fn pink() -> Color {
    color(255, 192, 203)
}
pub fn violet() -> Color {
    color(238, 130, 238)
}
pub fn dark_blue() -> Color {
    color(0, 0, 139)
}
pub fn dark_gray() -> Color {
    color(169, 169, 169)
}
pub fn dark_green() -> Color {
    color(0, 100, 0)
}
pub fn dark_red() -> Color {
    color(139, 0, 0)
}
pub fn light_blue() -> Color {
    color(173, 216, 230)
}
pub fn light_gray() -> Color {
    color(211, 211, 211)
}
pub fn light_green() -> Color {
    color(144, 238, 144)
}
pub fn light_red() -> Color {
    color(255, 204, 203)
}

use crate::types;

type Color = types::color::Color;
type ColorU8 = types::color::ColorU8;

pub fn u8_color(c: Color) -> ColorU8 {
    const _255: f32 = f32(255.0);

    return ColorU8 {
        r: u8(c.r * _255),
        g: u8(c.g * _255),
        b: u8(c.b * _255),
        a: u8(c.a * _255),
    }
}

pub fn f32_color(c: ColorU8) -> Color {
        const _255: f32 = f32(255.0);

    return Color {
        r: f32(c.r) / _255,
        g: f32(c.g) / _255,
        b: f32(c.b) / _255,
        a: f32(c.a) / _255,
    }
}

pub fn color_u8_from_hex(hex: int) -> ColorU8 {
    const r:u8 = u8((hex >> 16) & 0xFF);
    const g:u8 = u8((hex >> 8) & 0xFF);
    const b:u8 = u8((hex >> 0) & 0xFF);
    return ColorU8{
        r: r,
        g: g,
        b: b,
        a: 255
    }
}

pub fn color_from_hex(hex: int) -> Color {
    let col_u8 = color_u8_from_hex(hex)
    return f32_color(col_u8)
}

pub fn color_from_rgb(r: u8, g: u8, b: u8) -> Color {
    return Color {
        r: f32(r) / 255.0,
        g: f32(g) / 255.0,
        b: f32(b) / 255.0,
        a: 255,
    }
}

pub fn color_from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    return Color {
        r: f32(r) / 255.0,
        g: f32(g) / 255.0,
        b: f32(b) / 255.0,
        a: f32(a) / 255.0,
    }
}

pub fn color_u8_from_rgb(r: u8, g: u8, b: u8) -> ColorU8 {
    return ColorU8 {
        r: r,
        g: g,
        b: b,
        a: 255
    }
}

pub fn color_u8_from_rgba(r: u8, g: u8, b: u8, a: u8) -> ColorU8 {
    return ColorU8 {
        r: r,
        g: g,
        b: b,
        a: a,
    }
}


pub const BLACK:Color = color(0, 0, 0);
pub const WHITE:Color = color(255, 255, 255);
pub const GRAY:Color = color(128, 128, 128);
pub const RED:Color = color(255, 0, 0);
pub const GREEN:Color = color(0, 255, 0);
pub const BLUE:Color = color(0, 0, 255);

pub const YELLOW:Color    = color(255, 255, 0);
pub const MAGENTA:Color   = color(255, 0, 255);
pub const CYAN:Color      = color(0, 255, 255);
pub const ORANGE:Color    = color(255, 165, 0);
pub const PURPLE:Color    = color(128, 0, 128);
pub const INDIGO:Color    = color(75, 0, 130);
pub const PINK:Color      = color(255, 192, 203);
pub const VIOLET:Color    = color(238, 130, 238);
pub const DARK_BLUE:Color = color(0, 0, 139);
pub const DARK_GRAY:Color = color(169, 169, 169);
pub const DARK_GREEN:Color = color(0, 100, 0);
pub const DARK_RED:Color  = color(139, 0, 0);
pub const LIGHT_BLUE:Color = color(173, 216, 230);
pub const LIGHT_GRAY:Color = color(211, 211, 211);
pub const LIGHT_GREEN:Color = color(144, 238, 144);
pub const LIGHT_RED:Color  = color(255, 204, 203);

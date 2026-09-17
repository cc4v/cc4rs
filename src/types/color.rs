// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sokol::{gfx as sg};

pub struct ColorU8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub type Color = sg::Color;

pub fn default_color() -> Color {
    return Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0
    }
}

pub fn default_color_u8() -> ColorU8 {
    return ColorU8 {
        r: 0,
        g: 0,
        b: 00,
        a: 255,
    }
}

pub fn color(r: u8, g: u8, b: u8, a :u8) -> Color {
    return Color {
        r: (r as f32) / 255.0,
        g: (g as f32) / 255.0,
        b: (b as f32) / 255.0,
        a: (a as f32) / 255.0,
    }
}

macro_rules! color {
    ($r: expr, $g: expr, $b: expr) => {
        color($r, $g, $b, 255)
    };
    ($r: expr, $g: expr, $b: expr, $a: expr) => {
        color($r, $g, $b, $a)
    };
}

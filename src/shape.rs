// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{with_current_cc, Color};
use sokol::gl as sgl;

pub fn set_color(color: Color) {
    with_current_cc(|cc| {
        cc.current_style.color = color;
        cc.current_style.text_config.color = color;
    });
}

pub fn fill() {
    with_current_cc(|cc| cc.current_style.fill = true);
}

pub fn no_fill() {
    with_current_cc(|cc| cc.current_style.fill = false);
}

pub fn line(x1: f32, y1: f32, x2: f32, y2: f32) {
    let Some(color) = with_current_cc(|cc| cc.current_style.color) else {
        return;
    };

    sgl::c4f(color.r, color.g, color.b, color.a);
    sgl::begin_lines();
    sgl::v2f(x1, y1);
    sgl::v2f(x2, y2);
    sgl::end();
}

pub fn square(x: f32, y: f32, size: f32) {
    rect(x, y, size, size);
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

pub fn circle(x: f32, y: f32, radius: f32) {
    let Some((color, is_filled, resolution)) = with_current_cc(|cc| {
        (
            cc.current_style.color,
            cc.current_style.fill,
            cc.current_style.circle_resolution.max(3) as usize,
        )
    }) else {
        return;
    };

    sgl::c4f(color.r, color.g, color.b, color.a);
    if is_filled {
        sgl::begin_triangles();
        for index in 0..resolution {
            let angle = index as f32 * std::f32::consts::TAU / resolution as f32;
            let next_angle = (index + 1) as f32 * std::f32::consts::TAU / resolution as f32;
            sgl::v2f(x, y);
            sgl::v2f(x + radius * angle.cos(), y + radius * angle.sin());
            sgl::v2f(x + radius * next_angle.cos(), y + radius * next_angle.sin());
        }
    } else {
        sgl::begin_line_strip();
        for index in 0..=resolution {
            let angle = index as f32 * std::f32::consts::TAU / resolution as f32;
            sgl::v2f(x + radius * angle.cos(), y + radius * angle.sin());
        }
    }
    sgl::end();
}

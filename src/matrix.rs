// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sokol::gl as sgl;

pub fn scale(x: f32, y: f32, z: f32) {
    sgl::scale(x, y, z);
}

pub fn translate(x: f32, y: f32, z: f32) {
    sgl::translate(x, y, z);
}

pub fn rotate(angle_rad: f32, x: f32, y: f32, z: f32) {
    sgl::rotate(angle_rad, x, y, z);
}

pub fn rotate_deg(angle_deg: f32, x: f32, y: f32, z: f32) {
    rotate(sgl::rad(angle_deg), x, y, z);
}

pub fn rotate_z(angle_rad: f32) {
    rotate(angle_rad, 0.0, 0.0, 1.0);
}

pub fn rotate_z_deg(angle_deg: f32) {
    rotate_deg(angle_deg, 0.0, 0.0, 1.0);
}

pub fn rad(degrees: f32) -> f32 {
    sgl::rad(degrees)
}

pub fn deg(radians: f32) -> f32 {
    sgl::deg(radians)
}

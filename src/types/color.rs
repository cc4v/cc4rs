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

pub type Color = ColorU8;

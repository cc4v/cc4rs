// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[derive(Clone, Default)]
pub struct TextCfg {
    pub color: crate::Color,
    pub size: i32,
}

pub fn default_textcfg() -> TextCfg {
    return TextCfg {
        color: crate::default_color(),
        size: 16,
    };
}

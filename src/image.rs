// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{colors, with_current_cc};
use ::image::ImageError;
use sokol::{app as sapp, gfx as sg, gl as sgl};

pub struct Image {
    pub width: i32,
    pub height: i32,
    pub image: sg::Image,
    pub sampler: sg::Sampler,
    pub view: sg::View,
}

pub fn load_image(path: &str) -> Result<Image, ImageError> {
    let rgba = ::image::open(path)?.to_rgba8();
    let width = rgba.width() as i32;
    let height = rgba.height() as i32;
    let pixels = rgba.into_raw();

    let image = sg::make_image(&sg::ImageDesc {
        width,
        height,
        data: {
            let mut data = sg::ImageData::default();
            data.mip_levels[0] = sg::slice_as_range(&pixels);
            data
        },
        ..Default::default()
    });
    let sampler = sg::make_sampler(&sg::SamplerDesc {
        min_filter: sg::Filter::Linear,
        mag_filter: sg::Filter::Linear,
        wrap_u: sg::Wrap::ClampToEdge,
        wrap_v: sg::Wrap::ClampToEdge,
        ..Default::default()
    });
    let view = sg::make_view(&sg::ViewDesc {
        texture: sg::TextureViewDesc { image, ..Default::default() },
        ..Default::default()
    });

    Ok(Image { width, height, image, sampler, view })
}

pub fn delete_image(image: &mut Image) {
    sg::destroy_view(image.view);
    sg::destroy_sampler(image.sampler);
    sg::destroy_image(image.image);
    image.view = sg::View::default();
    image.sampler = sg::Sampler::default();
    image.image = sg::Image::default();
}

pub fn image(image: &Image, x: f32, y: f32) {
    image_with_size(image, x, y, image.width as f32, image.height as f32);
}

pub fn image_with_size(image: &Image, x: f32, y: f32, width: f32, height: f32) {
    let color = with_current_cc(|cc| cc.current_style.color).unwrap_or_else(colors::white);
    let pipeline = with_current_cc(|cc| cc.pipelines.alpha);

    if let Some(pipeline) = pipeline {
        sgl::load_pipeline(pipeline);
    }
    let rgba = colors::u8_color(color);
    sgl::enable_texture();
    sgl::texture(image.view, image.sampler);
    sgl::begin_quads();
    sgl::v2f_t2f_c4b(x, y, 0.0, 0.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::v2f_t2f_c4b(x + width, y, 1.0, 0.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::v2f_t2f_c4b(x + width, y + height, 1.0, 1.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::v2f_t2f_c4b(x, y + height, 0.0, 1.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::end();
    sgl::disable_texture();
}

pub fn image_3d(image: &Image, x: f32, y: f32, z: f32) {
    image_3d_with_size(image, x, y, z, image.width as f32, image.height as f32);
}

pub fn image_3d_with_size(
    image: &Image,
    x: f32,
    y: f32,
    z: f32,
    width: f32,
    height: f32,
) {
    let color = with_current_cc(|cc| cc.current_style.color).unwrap_or_else(colors::white);
    let pipeline = with_current_cc(|cc| cc.pipelines.alpha);
    if let Some(pipeline) = pipeline {
        sgl::load_pipeline(pipeline);
    }
    let rgba = colors::u8_color(color);
    sgl::enable_texture();
    sgl::texture(image.view, image.sampler);
    sgl::begin_quads();
    sgl::v3f_t2f_c4b(x, y, z, 0.0, 0.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::v3f_t2f_c4b(x + width, y, z, 1.0, 0.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::v3f_t2f_c4b(x + width, y + height, z, 1.0, 1.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::v3f_t2f_c4b(x, y + height, z, 0.0, 1.0, rgba.r, rgba.g, rgba.b, rgba.a);
    sgl::end();
    sgl::disable_texture();
}

pub fn window_size() -> (f32, f32) {
    (sapp::widthf(), sapp::heightf())
}

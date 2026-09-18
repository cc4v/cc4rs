// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sokol::gfx as sg;

pub struct Shader {
    raw: sg::Shader,
}

pub struct ShaderRect {
    shader: Shader,
    pipeline: sg::Pipeline,
    bindings: sg::Bindings,
}

impl Shader {
    pub fn from_backend(desc: fn(sg::Backend) -> sg::ShaderDesc) -> Self {
        let desc = desc(sg::query_backend());
        Self::from_desc(&desc)
    }

    pub fn from_desc(desc: &sg::ShaderDesc) -> Self {
        Self {
            raw: sg::make_shader(desc),
        }
    }

    pub fn raw(&self) -> sg::Shader {
        self.raw
    }

    pub fn make_pipeline(&self, mut desc: sg::PipelineDesc) -> sg::Pipeline {
        desc.shader = self.raw;
        sg::make_pipeline(&desc)
    }

    pub fn begin(&self, pipeline: sg::Pipeline, bindings: &sg::Bindings) {
        sg::apply_pipeline(pipeline);
        sg::apply_bindings(bindings);
    }

    pub fn set_uniform<T>(&self, slot: usize, value: &T) {
        sg::apply_uniforms(slot, &sg::value_as_range(value));
    }

    pub fn draw(&self, num_elements: usize) {
        sg::draw(0, num_elements, 1);
    }

    pub fn end(&self) {}

    pub fn is_loaded(&self) -> bool {
        self.raw.id != sg::INVALID_ID
    }

    pub fn unload(&mut self) {
        self.destroy();
    }

    pub fn destroy(&mut self) {
        if self.raw.id != sg::INVALID_ID {
            sg::destroy_shader(self.raw);
            self.raw = sg::Shader::default();
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.destroy();
    }
}

impl ShaderRect {
    pub fn from_backend(desc: fn(sg::Backend) -> sg::ShaderDesc) -> Self {
        Self::from_backend_with_position_attr(desc, 0)
    }

    pub fn from_backend_with_position_attr(
        desc: fn(sg::Backend) -> sg::ShaderDesc,
        position_attr: usize,
    ) -> Self {
        let shader = Shader::from_backend(desc);
        let mut pipeline_desc = sg::PipelineDesc {
            primitive_type: sg::PrimitiveType::TriangleStrip,
            ..Default::default()
        };
        pipeline_desc.layout.attrs[position_attr].format = sg::VertexFormat::Float2;
        let pipeline = shader.make_pipeline(pipeline_desc);
        let mut bindings = sg::Bindings::default();
        bindings.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
            size: std::mem::size_of::<[f32; 8]>(),
            usage: sg::BufferUsage {
                vertex_buffer: true,
                dynamic_update: true,
                ..Default::default()
            },
            ..Default::default()
        });
        Self {
            shader,
            pipeline,
            bindings,
        }
    }

    pub fn begin(&self) {
        self.shader.begin(self.pipeline, &self.bindings);
    }

    pub fn set_uniform<T>(&self, slot: usize, value: &T) {
        self.shader.set_uniform(slot, value);
    }

    pub fn rect(&self, x: f32, y: f32, width: f32, height: f32) {
        let screen_width = crate::width().max(1) as f32;
        let screen_height = crate::height().max(1) as f32;
        let left = x / screen_width * 2.0 - 1.0;
        let right = (x + width) / screen_width * 2.0 - 1.0;
        let top = 1.0 - y / screen_height * 2.0;
        let bottom = 1.0 - (y + height) / screen_height * 2.0;
        let vertices = [left, bottom, right, bottom, left, top, right, top];
        sg::update_buffer(self.bindings.vertex_buffers[0], &sg::slice_as_range(&vertices));
        self.shader.draw(4);
    }

    pub fn end(&self) {
        self.shader.end();
    }

    pub fn unload(&mut self) {
        sg::destroy_buffer(self.bindings.vertex_buffers[0]);
        sg::destroy_pipeline(self.pipeline);
        self.shader.unload();
        self.bindings.vertex_buffers[0] = sg::Buffer::default();
        self.pipeline = sg::Pipeline::default();
    }
}

impl Drop for ShaderRect {
    fn drop(&mut self) {
        if self.pipeline.id != sg::INVALID_ID {
            self.unload();
        }
    }
}
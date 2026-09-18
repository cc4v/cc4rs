// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sokol::gfx as sg;

pub struct Shader {
    raw: sg::Shader,
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
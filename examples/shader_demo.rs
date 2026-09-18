use cc4rs::*;
use sokol::gfx as sg;

#[path = "shader_demo/shader.rs"]
mod shader_demo_shader;

use shader_demo_shader::UB_FS_PARAMS;
use std::sync::{LazyLock, Mutex};

#[repr(C)]
#[derive(Clone, Copy)]
struct Params {
    time: f32,
    width: f32,
    height: f32,
    _padding: f32,
}

struct State {
    shader: Shader,
    pipeline: sg::Pipeline,
    bindings: sg::Bindings,
}

static STATE: LazyLock<Mutex<Option<State>>> = LazyLock::new(|| Mutex::new(None));

fn main() {
    on_init!(setup);
    on_exit!(cleanup);
    run(draw);
}

fn setup() {
    let shader = Shader::from_backend(shader_demo_shader::shader_demo_shader_desc);
    let mut pipeline_desc = sg::PipelineDesc {
        primitive_type: sg::PrimitiveType::TriangleStrip,
        ..Default::default()
    };
    pipeline_desc.layout.attrs[shader_demo_shader::ATTR_SHADER_DEMO_POSITION].format =
        sg::VertexFormat::Float2;
    let pipeline = shader.make_pipeline(pipeline_desc);
    let vertices: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
    let mut bindings = sg::Bindings::default();
    bindings.vertex_buffers[0] = sg::make_buffer(&sg::BufferDesc {
        data: sg::slice_as_range(&vertices),
        ..Default::default()
    });
    *STATE.lock().unwrap() = Some(State {
        shader,
        pipeline,
        bindings,
    });
}

fn draw() {
    let state_guard = STATE.lock().unwrap();
    let Some(state) = state_guard.as_ref() else {
        return;
    };
    let params = Params {
        time: elapsed_time() as f32,
        width: width().max(1) as f32,
        height: height().max(1) as f32,
        _padding: 0.0,
    };
    state.shader.begin(state.pipeline, &state.bindings);
    state.shader.set_uniform(UB_FS_PARAMS, &params);
    state.shader.draw(4);
    state.shader.end();
}

fn cleanup() {
    if let Some(mut state) = STATE.lock().unwrap().take() {
        sg::destroy_buffer(state.bindings.vertex_buffers[0]);
        sg::destroy_pipeline(state.pipeline);
        state.shader.destroy();
    }
}
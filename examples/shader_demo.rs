use cc4rs::*;
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
    shader: ShaderRect,
}

static STATE: LazyLock<Mutex<Option<State>>> = LazyLock::new(|| Mutex::new(None));

fn main() {
    on_init!(setup);
    on_exit!(cleanup);
    run(draw);
}

fn setup() {
    let shader = ShaderRect::from_backend(shader_demo_shader::shader_demo_shader_desc);
    *STATE.lock().unwrap() = Some(State {
        shader,
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
    state.shader.begin();
    state.shader.set_uniform(UB_FS_PARAMS, &params);
    state.shader.rect(0.0, 0.0, width() as f32, height() as f32);
    state.shader.end();
}

fn cleanup() {
    STATE.lock().unwrap().take();
}
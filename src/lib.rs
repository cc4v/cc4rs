// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sokol::{
    app as sapp,
    gfx::{self as sg, LoadAction, PassAction},
    gl as sgl, glue as sglue,
};
use stack_stack::Stack;
use std::{
    ffi,
    sync::{LazyLock, Mutex},
};

pub mod colors;
mod push_pop;
pub mod types;

pub use push_pop::*;

use crate::colors::color_from_rgba;

pub type Vector2<T> = crate::types::Vector2<T>;
pub type Vector3<T> = crate::types::Vector3<T>;
pub type Color = crate::types::Color;
pub type ColorU8 = crate::types::ColorU8;

pub fn u8_color(c: Color) -> ColorU8 {
    return crate::colors::u8_color(c);
}

pub fn f32_color(c: ColorU8) -> Color {
    return crate::colors::f32_color(c);
}

pub fn default_color() -> Color {
    return crate::types::color::default_color();
}

pub fn default_color_u8() -> ColorU8 {
    return crate::types::color::default_color_u8();
}

pub type TextCfg = crate::types::TextCfg;

pub fn default_textcfg() -> TextCfg {
    crate::types::default_textcfg()
}

pub type Event = sapp::Event;
pub type Keycode = sapp::Keycode;
pub type Mousebutton = sapp::Mousebutton;

// pub type RawPtr = *mut ffi::c_void;
// pub const NULLPTR: RawPtr = std::ptr::null_mut();

pub type RawPtr = usize;
pub const NULLPTR: RawPtr = 0;

pub type Modifiers = u32;

#[repr(u16)]
enum Modifier {
    SHIFT = 1,
    CTRL = 2,
    ALT = 4,
    SUPER = 8,
    LMB = 256,
    RMB = 512,
    MMB = 1024,
}

pub type FnCbWithPtr = fn(RawPtr);
pub type FnCbWithNoPtr = fn();
pub type FnMoveWithPtr = fn(f32, f32, RawPtr);
pub type FnMoveWithNoPtr = fn(f32, f32);
pub type FnEventWithPtr = fn(&sapp::Event, RawPtr);
pub type FnEventWithNoPtr = fn(&sapp::Event);
pub type FnKeyDownWithPtr = fn(sapp::Keycode, Modifiers, RawPtr);
pub type FnKeyDownWithNoPtr = fn(sapp::Keycode, Modifiers);
pub type FnKeyUpWithPtr = fn(sapp::Keycode, Modifiers, RawPtr);
pub type FnKeyUpWithNoPtr = fn(sapp::Keycode, Modifiers);
pub type FnClickWithPtr = fn(f32, f32, sapp::Mousebutton, RawPtr);
pub type FnClickWithNoPtr = fn(f32, f32, sapp::Mousebutton);
pub type FnUnClickWithPtr = fn(f32, f32, sapp::Mousebutton, RawPtr);
pub type FnUnClickWithNoPtr = fn(f32, f32, sapp::Mousebutton);

pub enum FnCb {
    FnCbWithPtr(FnCbWithPtr),
    FnCbWithNoPtr(FnCbWithNoPtr),
}

pub enum FnEvent {
    FnEventWithPtr(FnEventWithPtr),
    FnEventWithNoPtr(FnEventWithNoPtr),
}

pub enum FnMove {
    FnMoveWithPtr(FnMoveWithPtr),
    FnMoveWithNoPtr(FnMoveWithNoPtr),
}

pub enum FnKeyDown {
    FnKeyDownWithPtr(FnKeyDownWithPtr),
    FnKeyDownWithNoPtr(FnKeyDownWithNoPtr),
}

pub enum FnKeyUp {
    FnKeyUpWithPtr(FnKeyUpWithPtr),
    FnKeyUpWithNoPtr(FnKeyUpWithNoPtr),
}

pub enum FnClick {
    FnClickWithPtr(FnClickWithPtr),
    FnClickWithNoPtr(FnClickWithNoPtr),
}

pub enum FnUnClick {
    FnUnClickWithPtr(FnUnClickWithPtr),
    FnUnClickWithNoPtr(FnUnClickWithNoPtr),
}

pub type DrawFn = FnCb;

#[derive(Default)]
pub struct CCConfig {
    pub init_fn: Option<FnCb>,
    pub update_fn: Option<FnCb>,
    pub draw_fn: Option<FnCb>,
    pub cleanup_fn: Option<FnCb>,
    pub event_fn: Option<FnEvent>,
    pub keydown_fn: Option<FnKeyDown>,
    pub keyup_fn: Option<FnKeyUp>,
    pub click_fn: Option<FnClick>,
    pub unclick_fn: Option<FnUnClick>,
    pub move_fn: Option<FnMove>,
    pub user_data: RawPtr,
}

#[derive(Default)]
pub struct CCStyle {
    pub color: Color, // = gg.black,
    pub text_config: TextCfg,
    pub fill: bool, // = true,
    pub circle_resolution: i32,
    pub sphere_resolution: i32, // = 32,
    pub curve_resolution: i32,  // = 32
}

pub fn default_style() -> CCStyle {
    return CCStyle {
        color: colors::black(),
        text_config: default_textcfg(),
        fill: true,
        circle_resolution: 32,
        sphere_resolution: 32,
        curve_resolution: 32,
    };
}

#[derive(Default)]
pub struct CCPipelines {
    pub alpha: sgl::Pipeline,
    pub add: sgl::Pipeline,
}

#[derive(Default)]
pub struct InitialPreference {
    pub size: Option<Vector2<i32>>,
    pub init_fn: Option<FnCb>,
    pub cleanup_fn: Option<FnCb>,
    pub event_fn: Option<FnEvent>,
    pub keydown_fn: Option<FnKeyDown>,
    pub keyup_fn: Option<FnKeyUp>,
    pub click_fn: Option<FnClick>,
    pub unclick_fn: Option<FnUnClick>,
    pub move_fn: Option<FnMove>,
    pub bg_color: Option<Color>,
    pub title: String, // = "Canvas"
    pub fullscreen: bool,
    pub user_data: RawPtr,
}

#[derive(Default)]
pub struct CCState {
    pub pass_action: sg::PassAction,
    pub tex_view: sg::View,
    pub smp: sg::Sampler,
    pub pip_3d: sgl::Pipeline,
}

#[derive(Default)]
pub struct CCContext {
    pub cc: Option<&'static CC>,
    pub pref: InitialPreference,
}

#[derive(Default)]
pub struct CC {
    pub config: CCConfig,
    pub state: Option<&'static CCState>,
    pub current_style: CCStyle,
    pub style_history: Stack<CCStyle, CCMaxStyleHistory>,
    pub pipelines: CCPipelines,
    // pub fullscreen:     bool,
    // pub image_cache:    [dynamic]Image,
    pub img_count: usize,
    pub window_title_cstr: std::ffi::CString,
    pub width: usize,
    pub height: usize,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub last_modifiers: Modifiers,
    pub prev_modifiers: Modifiers,
    pub last_keycode: sapp::Keycode,
    pub prev_keycode: sapp::Keycode,
    pub last_keydown: bool,
    pub prev_keydown: bool,
    pub last_mousebutton: sapp::Mousebutton,
    pub prev_mousebutton: sapp::Mousebutton,
    pub last_mousedown: bool,
    pub prev_mousedown: bool,
}

static G_CTX: LazyLock<Mutex<CCContext>> = LazyLock::new(|| Mutex::new(CCContext::default()));
static STATE: LazyLock<Mutex<CCState>> = LazyLock::new(|| {
    let mut s = CCState::default();
    s.pass_action.colors[0].load_action = LoadAction::Clear;
    s.pass_action.colors[0].clear_value = color_from_rgba(0, 0, 0, 255);

    return Mutex::new(s);
});

fn get_context() -> std::sync::MutexGuard<'static, CCContext> {
    G_CTX.lock().unwrap()
}

fn ctx() -> std::sync::MutexGuard<'static, CCContext> {
    get_context()
}

// extern "C" fn cleanup(user_data: *mut ffi::c_void) {
//     sg::shutdown();

//     let _ = unsafe { Box::from_raw(user_data as *mut State) };
// }

fn setup(config: CCConfig) {
    let ctx = ctx();

    let w = 400;
    let h = 400;

    let bg_color = colors::white();
}

pub fn run(draw_fn: FnCbWithNoPtr) {
    setup(CCConfig {
        draw_fn: Some(FnCb::FnCbWithNoPtr(draw_fn)),
        user_data: NULLPTR,
        ..Default::default()
    });
}

pub fn run_with_data(draw_fn: FnCbWithPtr, user_data: RawPtr) {
    setup(CCConfig {
        draw_fn: Some(FnCb::FnCbWithPtr(draw_fn)),
        user_data,
        ..Default::default()
    });
}

#[macro_export]
macro_rules! run {
    ($draw_fn:expr) => {
        $crate::run($draw_fn as $crate::FnCbWithNoPtr)
    };
    ($draw_fn:expr, $user_data:expr) => {
        $crate::run_with_data($draw_fn as $crate::FnCbWithPtr, $user_data as rawptr)
    };
}

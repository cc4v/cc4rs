// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sokol::{
    app as sapp, debugtext as sdtx,
    gfx::{self as sg, LoadAction, PassAction},
    gl as sgl, glue as sglue, log as slog,
};
use stack_stack::Stack;
use std::{
    ffi,
    sync::{LazyLock, Mutex},
};

pub mod colors;
mod push_pop;
pub mod shape;
pub mod text;
pub mod types;

pub use push_pop::*;
pub use shape::*;
pub use text::*;

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

#[derive(Clone, Copy)]
pub enum FnCb {
    FnCbWithPtr(FnCbWithPtr),
    FnCbWithNoPtr(FnCbWithNoPtr),
}

#[derive(Clone, Copy)]
pub enum FnEvent {
    FnEventWithPtr(FnEventWithPtr),
    FnEventWithNoPtr(FnEventWithNoPtr),
}

#[derive(Clone, Copy)]
pub enum FnMove {
    FnMoveWithPtr(FnMoveWithPtr),
    FnMoveWithNoPtr(FnMoveWithNoPtr),
}

#[derive(Clone, Copy)]
pub enum FnKeyDown {
    FnKeyDownWithPtr(FnKeyDownWithPtr),
    FnKeyDownWithNoPtr(FnKeyDownWithNoPtr),
}

#[derive(Clone, Copy)]
pub enum FnKeyUp {
    FnKeyUpWithPtr(FnKeyUpWithPtr),
    FnKeyUpWithNoPtr(FnKeyUpWithNoPtr),
}

#[derive(Clone, Copy)]
pub enum FnClick {
    FnClickWithPtr(FnClickWithPtr),
    FnClickWithNoPtr(FnClickWithNoPtr),
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Default)]
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
    pub cc: Option<CC>,
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

static PREV_SIZE: LazyLock<Mutex<(usize, usize)>> = LazyLock::new(|| Mutex::new((0, 0)));
fn prev_size() -> std::sync::MutexGuard<'static, (usize, usize)> {
    PREV_SIZE.lock().unwrap()
}

fn state() -> std::sync::MutexGuard<'static, CCState> {
    STATE.lock().unwrap()
}

fn begin() {
    let dw = sapp::width();
    let dh = sapp::height();
    // sgl::viewport(0, 0, dw, dh, true);
    sgl::defaults();
    sgl::matrix_mode_projection();
    sgl::ortho(0.0, (dw as f32), (dh as f32), 0.0, -1.0, 1.0)
}

fn end() {
    sg::begin_pass(&sg::Pass {
        action: state().pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sgl::draw(); // FIXME: position/layer
    sdtx::draw(); // FIXME: position/layer
    sg::end_pass();
    sg::commit();
}

extern "C" fn init(_user_data: *mut ffi::c_void) {
    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
    sgl::setup(&sgl::Desc {
        logger: sgl::Logger { func: Some(slog::slog_func), ..Default::default() },
        ..Default::default()
    });
    let mut text_desc = sdtx::Desc::default();
    text_desc.fonts[0] = sdtx::font_oric();
    text_desc.logger.func = Some(slog::slog_func);
    sdtx::setup(&text_desc);

    with_current_cc(|cc| {
        if let Some(callback) = &cc.config.init_fn {
            match callback {
                FnCb::FnCbWithPtr(callback) => callback(cc.config.user_data),
                FnCb::FnCbWithNoPtr(callback) => callback(),
            }
        }
    });
}

extern "C" fn frame(_user_data: *mut ffi::c_void) {
    let update = {
        let context = ctx();
        context
            .cc
            .as_ref()
            .map_or((None, NULLPTR), |cc| (cc.config.update_fn, cc.config.user_data))
    };
    invoke_callback(update.0, update.1);

    begin();

    text::init_frame();
    push_matrix();
    push_style();

    let draw = {
        let context = ctx();
        context
            .cc
            .as_ref()
            .map_or((None, NULLPTR), |cc| (cc.config.draw_fn, cc.config.user_data))
    };
    invoke_callback(draw.0, draw.1);

    pop_style();
    pop_matrix();
    end();

    with_current_cc(|cc| {
        cc.prev_keycode = cc.last_keycode;
        cc.prev_keydown = cc.last_keydown;
        cc.prev_mousebutton = cc.last_mousebutton;
        cc.prev_mousedown = cc.last_mousedown;
        cc.prev_modifiers = cc.last_modifiers;
    });
}

fn invoke_callback(callback: Option<FnCb>, user_data: RawPtr) {
    match callback {
        Some(FnCb::FnCbWithPtr(callback)) => callback(user_data),
        Some(FnCb::FnCbWithNoPtr(callback)) => callback(),
        None => {}
    }
}

extern "C" fn cleanup(_user_data: *mut ffi::c_void) {
    with_current_cc(|cc| {
        if let Some(callback) = &cc.config.cleanup_fn {
            match callback {
                FnCb::FnCbWithPtr(callback) => callback(cc.config.user_data),
                FnCb::FnCbWithNoPtr(callback) => callback(),
            }
        }
    });

    sdtx::shutdown();
    sgl::shutdown();
    sg::shutdown();
}

extern "C" fn event(event: *const sapp::Event, _user_data: *mut ffi::c_void) {
    let Some(event) = (!event.is_null()).then(|| unsafe { &*event }) else {
        return;
    };
    let Some(()) = with_current_cc(|cc| {

    if let Some(callback) = &cc.config.event_fn {
        match callback {
            FnEvent::FnEventWithPtr(callback) => callback(event, cc.config.user_data),
            FnEvent::FnEventWithNoPtr(callback) => callback(event),
        }
    }

    match event._type {
        sapp::EventType::MouseDown
            if !cc.last_mousedown || cc.last_mousebutton != event.mouse_button => {
            if let Some(callback) = &cc.config.click_fn {
                match callback {
                    FnClick::FnClickWithPtr(callback) => callback(
                        event.mouse_x,
                        event.mouse_y,
                        event.mouse_button,
                        cc.config.user_data,
                    ),
                    FnClick::FnClickWithNoPtr(callback) => {
                        callback(event.mouse_x, event.mouse_y, event.mouse_button)
                    }
                }
            }
        }
        sapp::EventType::MouseUp
            if cc.last_mousedown || cc.last_mousebutton != event.mouse_button => {
            if let Some(callback) = &cc.config.unclick_fn {
                match callback {
                    FnUnClick::FnUnClickWithPtr(callback) => callback(
                        event.mouse_x,
                        event.mouse_y,
                        event.mouse_button,
                        cc.config.user_data,
                    ),
                    FnUnClick::FnUnClickWithNoPtr(callback) => {
                        callback(event.mouse_x, event.mouse_y, event.mouse_button)
                    }
                }
            }
        }
        sapp::EventType::MouseMove => {
            if let Some(callback) = &cc.config.move_fn {
                match callback {
                    FnMove::FnMoveWithPtr(callback) => {
                        callback(event.mouse_x, event.mouse_y, cc.config.user_data)
                    }
                    FnMove::FnMoveWithNoPtr(callback) => callback(event.mouse_x, event.mouse_y),
                }
            }
        }
        sapp::EventType::KeyDown
            if !cc.last_keydown
                || cc.last_keycode != event.key_code
                || cc.last_modifiers != event.modifiers => {
            if let Some(callback) = &cc.config.keydown_fn {
                match callback {
                    FnKeyDown::FnKeyDownWithPtr(callback) => {
                        callback(event.key_code, event.modifiers, cc.config.user_data)
                    }
                    FnKeyDown::FnKeyDownWithNoPtr(callback) => {
                        callback(event.key_code, event.modifiers)
                    }
                }
            }
        }
        sapp::EventType::KeyUp
            if cc.last_keydown
                || cc.last_keycode != event.key_code
                || cc.last_modifiers != event.modifiers => {
            if let Some(callback) = &cc.config.keyup_fn {
                match callback {
                    FnKeyUp::FnKeyUpWithPtr(callback) => {
                        callback(event.key_code, event.modifiers, cc.config.user_data)
                    }
                    FnKeyUp::FnKeyUpWithNoPtr(callback) => {
                        callback(event.key_code, event.modifiers)
                    }
                }
            }
        }
        _ => {}
    }

    if event._type == sapp::EventType::MouseScroll {
        cc.scroll_x = event.scroll_x;
        cc.scroll_y = event.scroll_y;
    } else {
        cc.scroll_x = 0.0;
        cc.scroll_y = 0.0;
    }
    if event._type == sapp::EventType::MouseMove {
        cc.mouse_x = event.mouse_x;
        cc.mouse_y = event.mouse_y;
        cc.mouse_dx = event.mouse_dx;
        cc.mouse_dy = event.mouse_dy;
    } else {
        cc.mouse_dx = 0.0;
        cc.mouse_dy = 0.0;
    }
    if matches!(event._type, sapp::EventType::KeyDown | sapp::EventType::KeyUp) {
        cc.last_keycode = event.key_code;
        cc.last_modifiers = event.modifiers;
    }
    if event._type == sapp::EventType::KeyDown {
        cc.last_keydown = true;
    } else if event._type == sapp::EventType::KeyUp {
        cc.last_keydown = false;
    }
    if event._type == sapp::EventType::MouseDown {
        cc.last_mousebutton = event.mouse_button;
        cc.last_mousedown = true;
    } else if event._type == sapp::EventType::MouseUp {
        cc.last_mousebutton = event.mouse_button;
        cc.last_mousedown = false;
    }
    if event._type == sapp::EventType::Resized {
        cc.width = event.window_width.max(0) as usize;
        cc.height = event.window_height.max(0) as usize;
    }
    }) else {
        return;
    };
}

fn get_context() -> std::sync::MutexGuard<'static, CCContext> {
    G_CTX.lock().unwrap()
}

fn with_current_cc<R>(callback: impl FnOnce(&mut CC) -> R) -> Option<R> {
    let mut context = get_context();
    context.cc.as_mut().map(callback)
}

fn ctx() -> std::sync::MutexGuard<'static, CCContext> {
    get_context()
}

// extern "C" fn cleanup(user_data: *mut ffi::c_void) {
//     sg::shutdown();

//     let _ = unsafe { Box::from_raw(user_data as *mut State) };
// }

fn setup(config: CCConfig) {
    let mut context = ctx();
    let preference = std::mem::take(&mut context.pref);

    let width = preference.size.map_or(400, |size| size.x as i32);
    let height = preference.size.map_or(400, |size| size.y as i32);
    let bg_color = preference.bg_color.unwrap_or_else(colors::white);
    let title = if preference.title.is_empty() {
        String::from("Canvas")
    } else {
        preference.title
    };

    let mut cc_config = config;
    if cc_config.user_data == NULLPTR {
        cc_config.user_data = preference.user_data;
    }
    if cc_config.init_fn.is_none() {
        cc_config.init_fn = preference.init_fn;
    }
    if cc_config.cleanup_fn.is_none() {
        cc_config.cleanup_fn = preference.cleanup_fn;
    }
    if cc_config.event_fn.is_none() {
        cc_config.event_fn = preference.event_fn;
    }
    if cc_config.keydown_fn.is_none() {
        cc_config.keydown_fn = preference.keydown_fn;
    }
    if cc_config.keyup_fn.is_none() {
        cc_config.keyup_fn = preference.keyup_fn;
    }
    if cc_config.click_fn.is_none() {
        cc_config.click_fn = preference.click_fn;
    }
    if cc_config.unclick_fn.is_none() {
        cc_config.unclick_fn = preference.unclick_fn;
    }
    if cc_config.move_fn.is_none() {
        cc_config.move_fn = preference.move_fn;
    }

    state().pass_action.colors[0].clear_value = bg_color;

    let window_title_cstr = ffi::CString::new(title).expect("window title contains NUL");
    let cc = CC {
        config: cc_config,
        current_style: default_style(),
        state: None,
        style_history: Stack::default(),
        pipelines: CCPipelines::default(),
        window_title_cstr,
        width: width as usize,
        height: height as usize,
        ..Default::default()
    };
    context.cc = Some(cc);
    drop(context);

    let mut desc = sapp::Desc::new();
    desc.width = width;
    desc.height = height;
    desc.fullscreen = preference.fullscreen;
    let context = get_context();
    let cc = context.cc.as_ref().expect("cc context must be initialized");
    desc.window_title = cc.window_title_cstr.as_ptr();
    desc.init_userdata_cb = Some(init);
    desc.frame_userdata_cb = Some(frame);
    desc.event_userdata_cb = Some(event);
    desc.cleanup_userdata_cb = Some(cleanup);
    desc.user_data = cc.config.user_data as *mut ffi::c_void;
    drop(context);

    sapp::run(&desc);
}

pub fn on_init(init_fn: FnCb) {
    let mut ctx = get_context();
    ctx.pref.init_fn = Some(init_fn)
}

pub fn on_event(event_fn: FnEvent) {
    let mut ctx = get_context();
    ctx.pref.event_fn = Some(event_fn)
}

pub fn on_exit(exit_fn: FnCb) {
    let mut ctx = get_context();
    ctx.pref.cleanup_fn = Some(exit_fn)
}

pub fn on_key_pressed(keydown_fn: FnKeyDown) {
    let mut ctx = get_context();
    ctx.pref.keydown_fn = Some(keydown_fn)
}

pub fn on_key_released(keyup_fn: FnKeyUp) {
    let mut ctx = get_context();
    ctx.pref.keyup_fn = Some(keyup_fn)
}

pub fn on_mouse_pressed(click_fn: FnClick) {
    let mut ctx = get_context();
    ctx.pref.click_fn = Some(click_fn)
}

pub fn on_mouse_released(unclick_fn: FnUnClick) {
    let mut ctx = get_context();
    ctx.pref.unclick_fn = Some(unclick_fn)
}

pub fn on_mouse_moved(move_fn: FnMove) {
    let mut ctx = get_context();
    ctx.pref.move_fn = Some(move_fn)
}

pub fn data() -> RawPtr {
    get_context()
        .cc
        .as_ref()
        .map_or(NULLPTR, |cc| cc.config.user_data)
}

pub fn set_data(user_data: RawPtr) {
    let mut context = get_context();
    if let Some(cc) = context.cc.as_mut() {
        cc.config.user_data = user_data;
    } else {
        context.pref.user_data = user_data;
    }
}

pub fn mouse_x() -> f32 {
    get_context().cc.as_ref().map_or(0.0, |cc| cc.mouse_x)
}

pub fn mouse_y() -> f32 {
    get_context().cc.as_ref().map_or(0.0, |cc| cc.mouse_y)
}

pub fn mouse_dx() -> f32 {
    get_context().cc.as_ref().map_or(0.0, |cc| cc.mouse_dx)
}

pub fn mouse_dy() -> f32 {
    get_context().cc.as_ref().map_or(0.0, |cc| cc.mouse_dy)
}

pub fn scroll_x() -> f32 {
    get_context().cc.as_ref().map_or(0.0, |cc| cc.scroll_x)
}

pub fn scroll_y() -> f32 {
    get_context().cc.as_ref().map_or(0.0, |cc| cc.scroll_y)
}

pub fn mouse_button() -> Mousebutton {
    get_context()
        .cc
        .as_ref()
        .map_or(Mousebutton::Invalid, |cc| cc.last_mousebutton)
}

pub fn mouse_pressed() -> bool {
    get_context().cc.as_ref().is_some_and(|cc| cc.last_mousedown)
}

pub fn mouse_released() -> bool {
    !mouse_pressed()
}

pub fn mouse_just_pressed(button: Mousebutton) -> bool {
    get_context().cc.as_ref().is_some_and(|cc| {
        cc.last_mousedown
            && cc.last_mousebutton == button
            && (cc.prev_mousebutton != cc.last_mousebutton || cc.prev_mousedown != cc.last_mousedown)
    })
}

pub fn mouse_just_released(button: Mousebutton) -> bool {
    get_context().cc.as_ref().is_some_and(|cc| {
        !cc.last_mousedown
            && cc.last_mousebutton == button
            && (cc.prev_mousebutton != cc.last_mousebutton || cc.prev_mousedown != cc.last_mousedown)
    })
}

pub fn key() -> Keycode {
    get_context()
        .cc
        .as_ref()
        .map_or(Keycode::Invalid, |cc| cc.last_keycode)
}

pub fn key_pressed() -> bool {
    get_context().cc.as_ref().is_some_and(|cc| cc.last_keydown)
}

pub fn key_released() -> bool {
    !key_pressed()
}

pub fn key_just_pressed(keycode: Keycode) -> bool {
    get_context().cc.as_ref().is_some_and(|cc| {
        cc.last_keydown
            && cc.last_keycode == keycode
            && (cc.prev_keycode != cc.last_keycode || cc.prev_keydown != cc.last_keydown)
    })
}

pub fn key_just_released(keycode: Keycode) -> bool {
    get_context().cc.as_ref().is_some_and(|cc| {
        !cc.last_keydown
            && cc.last_keycode == keycode
            && (cc.prev_keycode != cc.last_keycode || cc.prev_keydown != cc.last_keydown)
    })
}

pub fn toggle_fullscreen() {
    sapp::toggle_fullscreen();
}

#[macro_export]
macro_rules! on_init {
    ($callback:expr) => {
        $crate::on_init($crate::FnCb::FnCbWithNoPtr($callback))
    };
}

#[macro_export]
macro_rules! on_exit {
    ($callback:expr) => {
        $crate::on_exit($crate::FnCb::FnCbWithNoPtr($callback))
    };
}

#[macro_export]
macro_rules! on_key_pressed {
    ($callback:expr) => {
        $crate::on_key_pressed($crate::FnKeyDown::FnKeyDownWithNoPtr($callback))
    };
}

#[macro_export]
macro_rules! on_key_released {
    ($callback:expr) => {
        $crate::on_key_released($crate::FnKeyUp::FnKeyUpWithNoPtr($callback))
    };
}

#[macro_export]
macro_rules! on_mouse_pressed {
    ($callback:expr) => {
        $crate::on_mouse_pressed($crate::FnClick::FnClickWithNoPtr($callback))
    };
}

#[macro_export]
macro_rules! on_mouse_released {
    ($callback:expr) => {
        $crate::on_mouse_released($crate::FnUnClick::FnUnClickWithNoPtr($callback))
    };
}

#[macro_export]
macro_rules! on_mouse_moved {
    ($callback:expr) => {
        $crate::on_mouse_moved($crate::FnMove::FnMoveWithNoPtr($callback))
    };
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

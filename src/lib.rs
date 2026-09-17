// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{ffi, sync::{LazyLock, Mutex}};
use sokol::{app as sapp, gfx as sg, glue as sglue};

pub mod types;

pub type Vector2<T> = crate::types::vector::Vector2<T>;
pub type Vector3<T> = crate::types::vector::Vector3<T>;
pub type Color = crate::types::color::Color;

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
    MMB = 1024
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
    FnCbWithNoPtr(FnCbWithNoPtr)
}

pub enum FnEvent {
    FnEventWithPtr(FnEventWithPtr),
    FnEventWithNoPtr(FnEventWithNoPtr)
}

pub enum FnMove {
    FnMoveWithPtr(FnMoveWithPtr),
    FnMoveWithNoPtr(FnMoveWithNoPtr)
}

pub enum FnKeyDown {
    FnKeyDownWithPtr(FnKeyDownWithPtr),
    FnKeyDownWithNoPtr(FnKeyDownWithNoPtr)
}

pub enum FnKeyUp {
    FnKeyUpWithPtr(FnKeyUpWithPtr),
    FnKeyUpWithNoPtr(FnKeyUpWithNoPtr)
}

pub enum FnClick {
    FnClickWithPtr(FnClickWithPtr),
    FnClickWithNoPtr(FnClickWithNoPtr)
}

pub enum FnUnClick {
    FnUnClickWithPtr(FnUnClickWithPtr),
    FnUnClickWithNoPtr(FnUnClickWithNoPtr)
}

pub type DrawFn = FnCb;

#[derive(Default)]
pub struct CCConfig  {
	pub init_fn:      Option<FnCb>,
	pub update_fn:    Option<FnCb>,
	pub draw_fn:      Option<FnCb>,
	pub cleanup_fn:   Option<FnCb>,
	pub event_fn:     Option<FnEvent>,
	pub keydown_fn:   Option<FnKeyDown>,
	pub keyup_fn:     Option<FnKeyUp>,
	pub click_fn:     Option<FnClick>,
	pub unclick_fn:   Option<FnUnClick>,
	pub move_fn:      Option<FnMove>,
	pub user_data:    RawPtr,
}

#[derive(Default)]
pub struct InitialPreference {
    pub size:         Option<Vector2<i32>>,
	pub init_fn:      Option<FnCb>,
	pub cleanup_fn:   Option<FnCb>,
	pub event_fn:     Option<FnEvent>,
	pub keydown_fn:   Option<FnKeyDown>,
	pub keyup_fn:     Option<FnKeyUp>,
	pub click_fn:     Option<FnClick>,
	pub unclick_fn:   Option<FnUnClick>,
	pub move_fn:      Option<FnMove>,
	pub bg_color:     Option<Color>,
	pub title:        &'static str, // = "Canvas"
	pub fullscreen:   bool,
	pub user_data:    RawPtr
}

#[derive(Default)]
pub struct CCContext {
	pub cc:  Option<&'static CC>,
	pub pref: InitialPreference,
}

#[derive(Default)]
pub struct CC {
    pub config:         CCConfig,
	// state:          ^CCState,
}

static G_CTX: LazyLock<Mutex<CCContext>> = LazyLock::new(|| {
    Mutex::new(CCContext::default())
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

fn setup(config: CCConfig){
    let ctx = ctx();
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
        $crate::run_with_data(
            $draw_fn as $crate::FnCbWithPtr,
            $user_data as rawptr
        )
    };
}

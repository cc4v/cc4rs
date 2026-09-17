// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi;
use std::cell::LazyCell;
use sokol::{app as sapp, gfx as sg, glue as sglue};

pub mod types;

pub type Vector2<T> = crate::types::vector::Vector2<T>;
pub type Vector3<T> = crate::types::vector::Vector3<T>;
pub type Color = crate::types::color::Color;

pub type RawPtr = *mut ffi::c_void;
pub const NULLPTR: RawPtr = std::ptr::null_mut();

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
struct CCConfig  {
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
pub struct InitialPreference<'a> {
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
	pub title:        &'a str, // = "Canvas"
	pub fullscreen:   bool,
	pub user_data:    RawPtr
}

#[derive(Default)]
pub struct CCContext<'a, 'b> {
	pub cc:  &'a CC,
	pub pref: InitialPreference<'b>
}

#[derive(Default)]
pub struct CC {
    pub config:         CCConfig,
	// state:          ^CCState,
}

static G_CTX: LazyCell<CCContext> = LazyCell::new(|| {
    CCContext{}
});


// struct State {
//     pass_action: sg::PassAction,
// }

// extern "C" fn init(user_data: *mut ffi::c_void) {
//     let state = unsafe { &mut *(user_data as *mut State) };

//     sg::setup(&sg::Desc {
//         environment: sglue::environment(),
//         logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
//         ..Default::default()
//     });

//     state.pass_action.colors[0] = sg::ColorAttachmentAction {
//         load_action: sg::LoadAction::Clear,
//         clear_value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
//         ..Default::default()
//     };

//     let backend = sg::query_backend();
//     match &backend {
//         sg::Backend::Glcore | sg::Backend::Gles3 => {
//             println!("Using GL Backend!");
//             println!("Specifically the {:?} backend!", backend);
//         },

//         sg::Backend::D3d11 => {
//             println!("Using D3d11 Backend!");
//         },

//         sg::Backend::MetalIos | sg::Backend::MetalMacos | sg::Backend::MetalSimulator => {
//             println!("Using Metal Backend!");
//             println!("Specifically the {:?} backend!", backend);
//         },

//         sg::Backend::Wgpu => {
//             println!("Using Wgpu Backend!");
//         },

//         sg::Backend::Vulkan => {
//             println!("Using Vulkan Backend!");
//         },

//         sg::Backend::Dummy => {
//             println!("Using Dummy Backend!");
//         },
//     }
// }

// extern "C" fn frame(user_data: *mut ffi::c_void) {
//     let state = unsafe { &mut *(user_data as *mut State) };

//     let g = state.pass_action.colors[0].clear_value.g + 0.01;
//     state.pass_action.colors[0].clear_value.g = if g > 1.0 { 0.0 } else { g };

//     sg::begin_pass(&sg::Pass {
//         action: state.pass_action,
//         swapchain: sglue::swapchain(),
//         ..Default::default()
//     });
//     sg::end_pass();
//     sg::commit();
// }

fn get_context() -> &CCContext {
    return &*G_CTX;
}

fn ctx() -> &CCContext {
    return get_context();
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

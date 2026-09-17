// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi;

use sokol::{app as sapp, gfx as sg, glue as sglue};

pub type RawPtr = *mut ffi::c_void;
pub const NULLPTR: RawPtr = std::ptr::null_mut();

pub type Modifiers = u32;

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
	init_fn:      Option<FnCb>,
	update_fn:    Option<FnCb>,
	draw_fn:      Option<FnCb>,
	cleanup_fn:   Option<FnCb>,
	event_fn:     Option<FnEvent>,
	keydown_fn:   Option<FnKeyDown>,
	keyup_fn:     Option<FnKeyUp>,
	click_fn:     Option<FnClick>,
	unclick_fn:   Option<FnUnClick>,
	move_fn:      Option<FnMove>,
	user_data:    RawPtr,
}

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

// extern "C" fn cleanup(user_data: *mut ffi::c_void) {
//     sg::shutdown();

//     let _ = unsafe { Box::from_raw(user_data as *mut State) };
// }

fn setup(config: CCConfig){

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

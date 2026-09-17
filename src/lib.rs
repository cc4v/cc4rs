// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::ffi;

use sokol::{app as sapp, gfx as sg, glue as sglue};

pub type rawptr = *mut ffi::c_void;
pub type FnCb_WithPtr = fn(rawptr);
pub type FnCb_WithNoPtr = fn();

pub enum FnCb {
    FnCb_WithPtr(FnCb_WithPtr),
    FnCb_WithNoPtr(FnCb_WithNoPtr)
}

pub type DrawFn = FnCb;

struct CCConfig  {
	// init_fn:      Option<FnCb>,
	// update_fn:    Option<FnCb>,
	draw_fn:      Option<FnCb>,
	// cleanup_fn:   Option<FnCb>,
	// event_fn:     Option<FnEvent>,
	// keydown_fn:   Option<FnKeyDown>,
	// keyup_fn:     Option<FnKeyUp>,
	// click_fn:     Option<FnClick>,
	// unclick_fn:   Option<FnUnClick>,
	// move_fn:      Option<FnMove>,
	user_data:    rawptr,
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

pub fn run(draw_fn: DrawFn) {
    setup(CCConfig {
        draw_fn: Some(draw_fn),
        user_data: std::ptr::null_mut(),
    });
}

pub fn run_with_data(draw_fn: DrawFn, user_data: rawptr) {
    setup(CCConfig {
        draw_fn: Some(draw_fn),
        user_data,
    });
}

#[macro_export]
macro_rules! run {
    ($draw_fn:expr) => {
        $crate::run($crate::FnCb::FnCb_WithNoPtr($draw_fn as $crate::FnCb_WithNoPtr))
    };
    ($draw_fn:expr, $user_data:expr) => {
        $crate::run_with_data(
            $crate::FnCb::FnCb_WithPtr($draw_fn as $crate::FnCb_WithPtr),
            $user_data
        )
    };
}

pub const CCMaxStyleHistory: usize = 32;

use crate::{with_current_cc, CC};
use sokol::gl as sgl;

pub fn push_matrix() {
	sgl::push_matrix();
}

pub fn pop_matrix() {
	sgl::pop_matrix();
}

fn apply_style(cc: &mut CC) {
	cc.current_style.text_config.color = cc.current_style.color;
}

pub fn push_style() {
	sgl::push_pipeline();
	with_current_cc(|cc| {
		let _ = cc.style_history.push(cc.current_style.clone());
	});
}

pub fn pop_style() {
	sgl::pop_pipeline();
	with_current_cc(|cc| {
		if let Some(style) = cc.style_history.pop() {
			cc.current_style = style;
			apply_style(cc);
		}
	});
}

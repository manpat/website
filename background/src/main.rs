#![feature(generators, generator_trait, box_syntax)]
#![feature(inclusive_range_syntax)]
#![feature(specialization)]
#![feature(ord_max_min)]
#![feature(link_args)]
#![feature(const_fn)]

extern crate common;

pub use common::*;

#[macro_use] pub mod bindings;
#[macro_use] pub mod coro_util;

pub mod mut_rc;

pub mod rendering;
pub mod webgl;

pub mod paper;
// pub mod particle;

pub use bindings::emscripten::*;
pub use coro_util::*;
pub use webgl::*;
pub use paper::*;

pub use rendering::gl;
pub use rendering::shader::*;

use std::mem::transmute;

pub static SHADER_VS: &'static str = include_str!("../assets/paper.vs");
pub static SHADER_FS: &'static str = include_str!("../assets/paper.fs");

fn main() {
	println!("main init");

	set_coro_as_main_loop(|| {
		let gl_ctx = WebGLContext::new();

		let mut screen_size = Vec2i::zero();

		unsafe {
			use std::ptr::null;

			let evt_ptr = transmute(&mut screen_size);

			on_resize(0, null(), evt_ptr);
			emscripten_set_resize_callback(null(), evt_ptr, 0, Some(on_resize));

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			gl_ctx.set_background(Color::grey_a(0.0, 0.0));
		}

		let shader = Shader::new(SHADER_VS, SHADER_FS);
		shader.use_program();

		let mut paper = Paper::new();

		loop {
			unsafe {
				gl::Clear(gl::COLOR_BUFFER_BIT);
				gl::Viewport(0, 0, screen_size.x, screen_size.y);
			}

			let aspect = screen_size.x as f32 / screen_size.y as f32;
			shader.set_proj(&Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0)));

			paper.clear();
			paper.build_circle(Vec2::zero(), 0.5, Color::grey_a(0.0, 0.5));
			paper.draw();

			yield;
		}
	});
}


unsafe extern "C"
fn on_resize(_: i32, _e: *const EmscriptenUiEvent, ud: *mut CVoid) -> i32 {
	let screen_size: &mut Vec2i = transmute(ud);

	js! { b"Module.canvas = document.getElementById('canvas')\0" };

	screen_size.x = js! { b"return (Module.canvas.width = Module.canvas.style.width = window.innerWidth)\0" };
	screen_size.y = js! { b"return (Module.canvas.height = Module.canvas.style.height = window.innerHeight)\0" };
	
	0
}
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

pub use bindings::emscripten::*;
pub use coro_util::*;
pub use webgl::*;
pub use paper::*;

pub use rendering::gl;
pub use rendering::shader::*;

use rand::Rng;

use std::mem::transmute;

pub static SHADER_VS: &'static str = include_str!("../assets/shader.vs");
pub static SHADER_FS: &'static str = include_str!("../assets/shader.fs");

struct Particle {
	pos: Vec2,
	size: f32,
	speed: f32,
	phase: f32,
}

fn main() {
	println!("main init");

	set_coro_as_main_loop(|| {
		let gl_ctx = WebGLContext::new();

		let mut screen_size = Vec2i::zero();
		let mut rng = rand::thread_rng();

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
		let mut particles = Vec::new();
		let mut spawn_timeout = 0.0f32;
		let mut elapsed_time = 0.0;

		loop {
			unsafe {
				gl::Viewport(0, 0, screen_size.x, screen_size.y);
			}

			let aspect = screen_size.x as f32 / screen_size.y as f32;
			shader.set_proj(&Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0)));
			shader.set_uniform_f32("u_time", elapsed_time);

			let dt = 1.0/60.0;
			let dir = Vec2::from_angle(PI/6.0);

			elapsed_time += dt;

			spawn_timeout -= dt;
			if spawn_timeout <= 0.0 {
				spawn_timeout = rng.gen_range(1.0, 5.0);

				let pos = (rng.gen::<Vec2>() * 2.0 - 1.0) * Vec2::new(aspect, 1.0);
				let size = rng.gen_range(0.2, 0.7);
				let speed = rng.gen_range(0.02, 0.05);

				particles.push(Particle {
					pos, size, speed,
					phase: 0.0
				});
			}

			for p in particles.iter_mut() {
				p.phase += dt / 6.0;
				p.pos = p.pos + dir * p.speed * dt;
			}

			paper.clear();
			for p in particles.iter() {
				let a = 0.5 * (p.phase*PI).sin().powf(0.5);
				paper.build_circle(p.pos, p.size, Color::grey_a(0.0, a));
			}
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
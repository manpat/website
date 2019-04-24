#![feature(box_syntax)]

use wasm_toys::common::*;

use wasm_toys::console_log;

use wasm_toys::{EngineClient, UpdateContext};

use wasm_toys::DT;
use wasm_toys::graphics::*;
use wasm_toys::graphics::vertex::ColorVertex as Vertex;

fn main() {
	wasm_toys::init_engine(box Background::new());
}

struct Background {
	program: gl::ProgramID,
	mesh: DynamicMesh<Vertex>,
}

impl Background {
	fn new() -> Background {
		let mut shader = include_str!("../engine/src/shaders/color.glsl")
			.split("/* @@@ */");

		let program = create_shader(
			shader.next().unwrap(),
			shader.next().unwrap(),
			&["position", "color"]
		);

		let color = Vec3::new(1.0, 0.0, 1.0);

		let mut mesh = DynamicMesh::new();

		mesh.add_quad(&[
			Vertex::new(Vec3::new(-0.5,-0.5, 0.0), color),
			Vertex::new(Vec3::new(-0.5, 0.5, 0.0), color),
			Vertex::new(Vec3::new( 0.5, 0.5, 0.0), color),
			Vertex::new(Vec3::new( 0.5,-0.5, 0.0), color),
		]);

		Background {
			program,
			mesh
		}
	}
}

impl EngineClient for Background {
	fn update(&mut self, ctx: UpdateContext) {
		unsafe {
			gl::use_program(self.program);
			// gl::clear_color(1.0, 0.0, 1.0, 0.1);
			// gl::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
		}

		gl::set_uniform_mat4(self.program, "proj_view", &Mat4::ident());

		self.mesh.draw(gl::DrawMode::Triangles);
	}
}
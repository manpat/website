#![feature(clamp)]

use wasm_toys::common::*;
use wasm_toys::console_log;
use wasm_toys::{EngineClient, UpdateContext};

use wasm_toys::DT;
use wasm_toys::graphics::*;
use wasm_toys::graphics::camera::*;

fn main() {
	wasm_toys::init_engine(Background::new());
}

struct Background {
	program: gl::ProgramID,
	mesh: DynamicMesh<Vertex>,
	camera: Camera,
}

impl Background {
	fn new() -> Background {
		let mut shader = include_str!("color_alpha.glsl")
			.split("/* @@@ */");

		let program = create_shader(
			shader.next().unwrap(),
			shader.next().unwrap(),
			&["position", "color"]
		);

		let mesh = DynamicMesh::new();

		let mut camera = Camera::new();
		camera.set_near_far(-1.0, 1000.0);
		camera.set_projection(Projection::Orthographic{ scale: 2.0 });

		let cam_ori = 
			  Quat::new(Vec3::from_y(1.0), PI/4.0)
			* Quat::new(Vec3::from_x(1.0), -PI/8.0);

		camera.set_orientation(cam_ori);
		camera.set_position(cam_ori.forward() * -3.0);

		Background {
			program,
			mesh,
			camera,
		}
	}
}

impl EngineClient for Background {
	fn update(&mut self, ctx: UpdateContext) {
		self.camera.update(ctx.viewport);

		let time = ctx.ticks as f32 * DT;

		let cam_ori = 
			  Quat::new(Vec3::from_y(1.0), PI/4.0 + time.sin()*0.05)
			* Quat::new(Vec3::from_x(1.0), -PI/8.0 + (time*0.4).cos() * 0.02);

		self.camera.set_orientation(cam_ori);
		self.camera.set_position(cam_ori.forward() * -3.0);

		unsafe {
			gl::use_program(self.program);
			gl::set_uniform_mat4(self.program, "proj_view", &self.camera.projection_view());
		}

		// Draw mask into depth buffer
		set_color_write(false);

		self.mesh.clear();
		self.mesh.add_quad(&[
			Vertex::new(Vec3::new(-1.0, 0.0, 1.0), Vec4::zero()),
			Vertex::new(Vec3::new( 1.0, 0.0, 1.0), Vec4::zero()),
			Vertex::new(Vec3::new( 1.0,-4.0, 1.0), Vec4::zero()),
			Vertex::new(Vec3::new(-1.0,-4.0, 1.0), Vec4::zero()),
		]);
		self.mesh.add_quad(&[
			Vertex::new(Vec3::new( 1.0, 0.0, 1.0), Vec4::zero()),
			Vertex::new(Vec3::new( 1.0, 0.0,-1.0), Vec4::zero()),
			Vertex::new(Vec3::new( 1.0,-4.0,-1.0), Vec4::zero()),
			Vertex::new(Vec3::new( 1.0,-4.0, 1.0), Vec4::zero()),
		]);

		let reveal_time = 2.0;

		let reveal_phase = ((time-0.5) / reveal_time).clamp(0.0, 1.0) * PI;
		let pos = reveal_phase.cos()*1.0 - 1.0;

		self.mesh.add_quad(&[
			Vertex::new(Vec3::new(-1.0, pos,-1.0), Vec4::zero()),
			Vertex::new(Vec3::new( 1.0, pos,-1.0), Vec4::zero()),
			Vertex::new(Vec3::new( 1.0, pos, 1.0), Vec4::zero()),
			Vertex::new(Vec3::new(-1.0, pos, 1.0), Vec4::zero()),
		]);

		self.mesh.draw(gl::DrawMode::Triangles);


		// Draw hole background
		set_color_write(true);

		let shaft_height = 1.4;

		self.mesh.clear();
		self.mesh.add_quad(&[
			Vertex::new(Vec3::new( 1.0,          0.0,-1.0), Color::grey_a(0.0, 0.1)),
			Vertex::new(Vec3::new(-1.0,          0.0,-1.0), Color::grey_a(0.0, 0.1)),
			Vertex::new(Vec3::new(-1.0,-shaft_height,-1.0), Color::grey_a(0.0, 0.5)),
			Vertex::new(Vec3::new( 1.0,-shaft_height,-1.0), Color::grey_a(0.0, 0.5)),
		]);

		self.mesh.add_quad(&[
			Vertex::new(Vec3::new(-1.0,          0.0,-1.0), Color::grey_a(0.0, 0.2)),
			Vertex::new(Vec3::new(-1.0,          0.0, 1.0), Color::grey_a(0.0, 0.2)),
			Vertex::new(Vec3::new(-1.0,-shaft_height, 1.0), Color::grey_a(0.0, 0.5)),
			Vertex::new(Vec3::new(-1.0,-shaft_height,-1.0), Color::grey_a(0.0, 0.5)),
		]);

		self.mesh.draw(gl::DrawMode::Triangles);
	}
}



#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
	pos: Vec3,
	color: Vec4,
}

impl Vertex {
	pub fn new<C>(pos: Vec3, color: C) -> Self where C: Into<Vec4> {
		Vertex{pos, color: color.into()}
	}
}

impl vertex::Vertex for Vertex {
	fn descriptor() -> vertex::Descriptor {
		vertex::Descriptor::from(&[3, 4])
	}
}

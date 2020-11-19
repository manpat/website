use wasm_toys::prelude::*;
use wasm_toys::imports::util::math_random;

fn random() -> f32 { unsafe{ math_random() * 2.0 - 1.0 } }
fn random_range(b: f32, e: f32) -> f32 { unsafe{ math_random() * (e-b) + b } }

const PARTICLE_LIFESPAN: f32 = 18.0;

pub struct ParticleSystem {
	program: gl::ProgramID,
	mesh: BasicDynamicMesh<ParticleVertex>,

	particles: Vec<Particle>,
	spawn_timer: f32,
}

struct Particle {
	position: Vec3,
	age: f32,
	heading: f32,

	offset: f32,
}

impl ParticleSystem {
	pub fn new() -> ParticleSystem {
		ParticleSystem {
			program: create_shader_combined(
				include_str!("point.glsl"),
				&["position_scale", "color"]
			),

			mesh: BasicDynamicMesh::new(),

			particles: Vec::new(),
			spawn_timer: 0.0,
		}
	}

	pub fn update(&mut self) {
		for p in self.particles.iter_mut() {
			let altitude = p.position.y;

			p.heading += random() * 0.2;
			let heading = Vec3::from_y_angle(p.heading);
			
			p.position += heading * (altitude+0.5).clamp(0.1, 1.0) / (2.0 + p.offset) * DT * (p.age/2.0).cos().abs();

			if altitude <= 0.0 {
				p.position.y += 0.4 * DT;
			} else {
				p.position.y += 0.4 * (0.9 - altitude) * DT;
			}

			p.position.y += (p.age*3.0*PI + p.offset).sin() * 0.2 * DT;

			p.age += DT;
		}

		self.particles.retain(|p| p.age < PARTICLE_LIFESPAN);

		self.spawn_timer -= DT;

		if self.spawn_timer <= 0.0 {
			self.particles.push(Particle {
				position: Vec3::new(random()*0.5, -1.5, random()*0.5),
				age: 0.0,

				heading: random_range(0.0, 2.0 * PI),

				offset: random_range(0.0, 1.0),
			});

			self.spawn_timer = random_range(0.1, 1.5);
		}
	}

	pub fn draw(&mut self, camera: &Camera) {
		if self.particles.is_empty() { return }

		let vp = camera.viewport().to_vec2();
		let part_scale = if camera.aspect() > 1.0 { vp.y/200.0 } else { vp.x/200.0 };

		unsafe {
			gl::use_program(self.program);
			gl::set_uniform_mat4(self.program, "proj_view", &camera.projection_view());
			gl::set_uniform_f32(self.program, "global_particle_scale", part_scale);
		}

		let fwd = camera.orientation().forward();
		self.particles.sort_by_cached_key(|p| ordify(&-p.position.dot(fwd)));

		self.mesh.clear();

		for p in self.particles.iter() {
			let alpha = 0.08 * (1.0 - p.age/PARTICLE_LIFESPAN).clamp(0.0, 1.0)
				* (1.0+p.position.y.min(0.0));
			let size = (8.0 + p.offset*4.0) * ((PARTICLE_LIFESPAN - p.age + 2.0)/20.0).clamp(0.0, 1.0);

			self.mesh.add_vertex(
				ParticleVertex::new(p.position, size, Color::grey_a(0.0, alpha))
			);
		}

		self.mesh.draw(gl::DrawMode::Points);
	}
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ParticleVertex {
	pos_scale: Vec4,
	color: Vec4,
}

impl ParticleVertex {
	pub fn new<C>(pos: Vec3, scale: f32, color: C) -> Self where C: Into<Vec4> {
		ParticleVertex{pos_scale: pos.extend(scale), color: color.into()}
	}
}

impl vertex::Vertex for ParticleVertex {
	fn descriptor() -> vertex::Descriptor {
		vertex::Descriptor::from(&[4, 4])
	}
}

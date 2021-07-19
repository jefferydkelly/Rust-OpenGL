use glm::{Vec3, vec3};

#[derive(Clone, Copy)]
pub struct DirectionalLight {
	pub direction:Vec3,
	pub ambient:Vec3,
	pub diffuse:Vec3,
	pub specular:Vec3
}

#[derive(Clone, Copy)]
pub struct PointLight {
	pub position:Vec3,
	
	pub ambient:Vec3,
	pub diffuse:Vec3,
	pub specular:Vec3,

	pub constant:f32,
	pub linear:f32,
	pub quadratic:f32
}

#[derive(Clone, Copy)]
pub struct Spotlight {
	pub cutoff:f32,
	pub outer_cutoff:f32,

    pub direction:Vec3,
	pub position:Vec3,
	pub ambient:Vec3,
	pub diffuse:Vec3,
	pub specular:Vec3,

	pub constant:f32,
	pub linear:f32,
	pub quadratic:f32
}
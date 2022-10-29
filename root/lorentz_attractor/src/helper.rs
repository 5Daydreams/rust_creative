use nannou::prelude::*;

use crate::vec_extensions::Perspective2D;

pub trait Nannou
{
	fn update(&mut self);
	fn display(&self, draw: &nannou::Draw);
}

pub struct Time
{
	pub curr_time: f32,
	pub prev_time: f32,
	pub delta_time: f32,
}

pub struct LorentzPoint
{
	pub near_plane: f32,
	pub far_plane: f32,
	pub fov_radians: f32,
	pub window_size: [u32; 2],
	pub delta_time: f32,
	pub lorentz_constants: Vec3,
	pub prev_point: Vec3,
	pub curr_point: Vec3,
	pub color: Srgb<f32>,
}

impl LorentzPoint
{
	pub fn refresh_time(&mut self, delta_time: f32) { self.delta_time = delta_time; }
}

impl Nannou for LorentzPoint
{
	fn update(&mut self)
	{
		self.prev_point = self.curr_point;

		let dt: f32 = self.delta_time;
		let point: Vec3 = self.curr_point;
		let l_cst: Vec3 = self.lorentz_constants;

		let movement_delta = Vec3::new(
			(l_cst.x * (point.y - point.x)) * dt,
			(point.x * (l_cst.z - point.z) - point.y) * dt,
			(point.x * point.y - l_cst.y * point.z) * dt,
		);

		self.curr_point += movement_delta;
	}

	fn display(&self, draw: &nannou::Draw)
	{
		let x_offset: f32 = 0.;
		let y_offset: f32 = 0.;
		let z_offset: f32 = 10.;

		let mut temp: Vec2;
		let offset: Vec3 = Vec3::new(x_offset, y_offset, z_offset);

		temp = (self.prev_point + offset).project_into_2d(
			self.window_size,
			self.fov_radians,
			self.near_plane,
			self.far_plane,
		);
		let draw_vec_prev: Vec2 = temp;

		temp = (self.curr_point + offset).project_into_2d(
			self.window_size,
			self.fov_radians,
			self.near_plane,
			self.far_plane,
		);
		let draw_vec_curr: Vec2 = temp;

		draw.line()
			.start(draw_vec_prev)
			.end(draw_vec_curr)
			.caps_round()
			.color(self.color);
	}
}

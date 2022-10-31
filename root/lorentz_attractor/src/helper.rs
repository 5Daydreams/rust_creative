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
	pub curr_time: f32,
	pub lorentz_constants: Vec3,
	pub prev_point: Vec3,
	pub curr_point: Vec3,
	pub color: Srgb<f32>,
}

impl LorentzPoint
{
	pub fn refresh_time(&mut self, delta_time: f32)
	{
		self.delta_time = delta_time;
		self.curr_time += 1. * self.delta_time;
	}
}

impl Nannou for LorentzPoint
{
	fn update(&mut self)
	{
		self.prev_point = self.curr_point;

		let dt: f32 = self.delta_time/20.;
		let point: Vec3 = self.curr_point;
		let l_cst: Vec3 = self.lorentz_constants;

		let movement_delta: Vec3 = Vec3::new(
			(l_cst.x * (point.y - point.x)) * dt,
			(point.x * (l_cst.z - point.z) - point.y) * dt,
			(point.x * point.y - l_cst.y * point.z) * dt,
		);

		self.curr_point += movement_delta;
	}

	fn display(&self, draw: &nannou::Draw)
	{
		let offset: Vec3 = Vec3::new(0., 0., 120.);

		let project_from_3d = |vec: Vec3| -> Vec2 {
			(vec).project_into_2d(
				offset,
				self.window_size,
				self.fov_radians,
				self.near_plane,
				self.far_plane,
				self.curr_time/5.,
			)
		};

		let mut temp_2: Vec2;

		temp_2 = project_from_3d(self.prev_point);
		let draw_vec_prev: Vec2 = temp_2;

		temp_2 = project_from_3d(self.curr_point);
		let draw_vec_curr: Vec2 = temp_2;

		draw.line()
			.start(draw_vec_prev)
			.end(draw_vec_curr)
			.caps_round()
			.color(self.color);
	}
}

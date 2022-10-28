use std::ops::Range;

use nannou::prelude::{
	Vec2,
	Vec3,
};
use rand::Rng;

pub trait Randomize
{
	fn randomize(&mut self, x_range: Range<f32>, y_range: Range<f32>, z_range: Range<f32>);
}

pub trait Perspective2D
{
	fn project_into_2d(&self, window_size: [u32;2], fov: f32, near_plane: f32, far_plane: f32)
		-> Vec2;
}

impl Randomize for Vec3
{
	fn randomize(&mut self, x_range: Range<f32>, y_range: Range<f32>, z_range: Range<f32>)
	{
		let new_x: f32 = rand::thread_rng().gen_range(x_range);
		self.x = new_x;

		let new_y: f32 = rand::thread_rng().gen_range(y_range);
		self.y = new_y;

		let new_z: f32 = rand::thread_rng().gen_range(z_range);
		self.z = new_z;
	}
}

impl Perspective2D for Vec3
{
	fn project_into_2d(
		&self,
		window_size: [u32;2],
		fov_radians: f32,
		near_plane: f32,
		far_plane: f32,
	) -> Vec2
	{
		let w = window_size[1] as f32;
		let h = window_size[0] as f32;

		let window_aspect_ratio: f32 = w / h;

		let inverse_tangent_of_fov: f32 = 1. / (fov_radians * 0.5).tan();
		let q_value: f32 = far_plane / (far_plane - near_plane);

		let projected_vec: Vec3 = Vec3::new(
			(window_aspect_ratio * inverse_tangent_of_fov * self.x) / self.z,
			(inverse_tangent_of_fov * self.y) / self.z,
			((self.z - near_plane) * q_value) / self.z,
		);

		Vec2::new(projected_vec.x * (w * 0.5), projected_vec.y * (h * 0.5))
	}
}

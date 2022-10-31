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
	fn project_into_2d(
		&self,
		offset: Vec3,
		window_size: [u32; 2],
		fov: f32,
		near_plane: f32,
		far_plane: f32,
		time: f32,
	) -> Vec2;
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
		offset: Vec3,
		window_size: [u32; 2],
		fov_radians: f32,
		near_plane: f32,
		far_plane: f32,
		time: f32,
	) -> Vec2
	{
		let w: f32 = window_size[1] as f32;
		let h: f32 = window_size[0] as f32;
		let window_aspect_ratio: f32 = w / h;

		let inverse_tangent_of_fov: f32 = 1. / (fov_radians * 0.5).tan();
		let q_value: f32 = far_plane / (far_plane - near_plane);

		let rot_y_closure = |vec: Vec3, rot_radians: f32| -> Vec3 {
			Vec3::new(
				rot_radians.cos() * vec.x - rot_radians.sin() * vec.z,
				vec.y,
				rot_radians.sin() * vec.x + rot_radians.cos() * vec.z,
			)
		};

		let proj_closure = |vec: Vec3| -> Vec3 {
			Vec3::new(
				(window_aspect_ratio * inverse_tangent_of_fov * vec.x) / vec.z,
				(inverse_tangent_of_fov * vec.y) / vec.z,
				((vec.z - near_plane) * q_value) / vec.z,
			)
		};

		let spin_vec: Vec3 = rot_y_closure(*self, time);

		let offset_vec: Vec3 = spin_vec + offset;

		let projected_vec: Vec3 = proj_closure(offset_vec);

		Vec2::new(projected_vec.x * (w * 0.5), projected_vec.y * (h * 0.5))
	}
}

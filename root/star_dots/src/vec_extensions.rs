use nannou::prelude::{
	Vec2,
	Vec3,
};

pub trait RotateMatrixBaked
{
	fn rotate_y(&self, rot_radians: f32) -> Vec3;
}

impl RotateMatrixBaked for Vec3
{
	fn rotate_y(&self, rot_radians: f32) -> Vec3
	{
		let rot_y_closure = |vec: Vec3, rot_radians: f32| -> Vec3 {
			Vec3::new(
				rot_radians.cos() * vec.x - rot_radians.sin() * vec.z,
				vec.y,
				rot_radians.sin() * vec.x + rot_radians.cos() * vec.z,
			)
		};

		rot_y_closure(*self, rot_radians)
	}
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
	) -> Vec2;
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
	) -> Vec2
	{
		let w: f32 = window_size[0] as f32;
		let h: f32 = window_size[1] as f32;
		let window_aspect_ratio: f32 = h / w;

		let inverse_tangent_of_fov: f32 = 1. / (fov_radians * 0.5).tan();
		let q_value: f32 = far_plane / (far_plane - near_plane);

		let proj_closure = |vec: Vec3| -> Vec3 {
			Vec3::new(
				(window_aspect_ratio * inverse_tangent_of_fov * vec.x) / vec.z,
				(inverse_tangent_of_fov * vec.y) / vec.z,
				((vec.z - near_plane) * q_value) / vec.z,
			)
		};

		let projected_vec: Vec3 = proj_closure(*self + offset);

		Vec2::new(projected_vec.x * (w * 0.5), projected_vec.y * (h * 0.5))
	}
}

use std::ops::Range;

use nannou::{
	glam::const_vec3,
	prelude::{
		Vec3,
		PI,
	},
};

pub const POINT_COUNT: u32 = 80;
pub const DEFAULT_WINDOW_SIZE: (u32, u32) = (800, 600);
pub const FOV: f32 = PI * 0.85;
pub const CLIPPING_PLANES: (f32, f32) = (0.1, 12.0);

pub const SPAWN_RANGE_X: Range<f32> = -3.0 .. 5.0;
pub const SPAWN_RANGE_Y: Range<f32> = -6.0 .. 4.0;
pub const SPAWN_RANGE_Z: Range<f32> = -3.5 .. 4.5;

pub const OFFSET_VEC: Vec3 = const_vec3!([0., 0., 6.,]);

use std::ops::Range;

use nannou::{
	glam::const_vec3,
	prelude::{
		Vec3,
		PI,
	},
};

pub const POINT_COUNT: u32 = 1500;
pub const DEFAULT_WINDOW_SIZE: (u32, u32) = (512, 512);
pub const FOV: f32 = PI * 0.55;
pub const CLIPPING_PLANES: (f32, f32) = (0.1, 12.0);

pub const SPAWN_RANGE_X: Range<f32> = -2.0 .. 2.0;
pub const SPAWN_RANGE_Y: Range<f32> = -3.0 .. 3.0;
pub const SPAWN_RANGE_Z: Range<f32> = -2.0 .. 2.0;

pub const OFFSET_VEC: Vec3 = const_vec3!([0., 0., 5.5,]);

pub const EDGE_STRENGTH: f32 = 1.2;
pub const NEIGHBOUR_COUNT_MAX: usize = 2;
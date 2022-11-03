use std::ops::Range;

use nannou::prelude::*;
use rand::Rng;

pub struct RangeCube
{
	pub range_x: Range<f32>,
	pub range_y: Range<f32>,
	pub range_z: Range<f32>,
}

impl RangeCube
{
	pub fn new(range_x: Range<f32>, range_y: Range<f32>, range_z: Range<f32>) -> RangeCube
	{
		RangeCube {
			range_x,
			range_y,
			range_z,
		}
	}

	pub fn get_random_vec3(&self) -> Vec3
	{
		Vec3::new(
			rand::thread_rng().gen_range(self.range_x.clone()),
			rand::thread_rng().gen_range(self.range_y.clone()),
			rand::thread_rng().gen_range(self.range_z.clone()),
		)
	}
}

use std::ops::Range;

use nannou::prelude::Vec2;
use rand::Rng;

pub trait Randomize
{
	fn randomize(&mut self, x_range: Range<f32>, y_range: Range<f32>);
}

impl Randomize for Vec2
{
	fn randomize(&mut self, x_range: Range<f32>, y_range: Range<f32>)
	{
		let new_x: f32 = rand::thread_rng().gen_range(x_range);
		self.x = new_x;

		let new_y: f32 = rand::thread_rng().gen_range(y_range);
		self.y = new_y;
	}
}

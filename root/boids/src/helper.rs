use nannou::prelude::*;

pub struct Boid
{
	pub position: Vec2,
	pub velocity: Vec2,
	pub acceleration: Vec2,
	pub radius: f32,
	pub color: Srgb<f32>,
}

impl Boid
{
	pub fn display(&self, draw: &nannou::Draw)
	{
		draw.ellipse()
			.x_y(self.position.x, self.position.y)
			.radius(self.radius)
			.color(self.color);
	}

	pub fn new() -> Self {  Default::default() }
}

impl Default for Boid
{
	fn default() -> Self
	{
		Self {
			position: Vec2::default(),
			velocity: Vec2::default(),
			acceleration: Vec2::default(),
			radius: 5.,
			color: Srgb::<f32>::new(0.7, 0.2, 0.6),
		}
	}
}

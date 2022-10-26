use nannou::prelude::*;

#[derive(PartialEq)]
pub struct Boid
{
	position: Vec2,
	velocity: Vec2,
	acceleration: Vec2,
	radius: f32,
	color: Srgb<f32>,
}

pub trait Nannou
{
	fn display(&self, draw: &nannou::Draw);
	fn update(&mut self);
}

#[warn(clippy::new_ret_no_self)]
impl Boid
{
	pub fn new() -> BoidBuilder
	{
		BoidBuilder {
			position: None,
			velocity: None,
			acceleration: None,
			radius: None,
			color: None,
		}
	}

	fn align(&mut self, other_boids: &Vec<Boid>) -> Vec2
	{
		let perception_radius: f32 = 100.;
		let mut target_velocity: Vec2 = Vec2::new(0., 0.);
		let mut neighbour_count: i32 = 0;

		for other in other_boids
		{
			let dist = Vec2::distance(self.position, other.position);
			if dist < perception_radius && other != self
			{
				target_velocity += other.velocity;
				neighbour_count += 1;
			}
		}

		if neighbour_count > 0
		{
			target_velocity /= neighbour_count as f32;
			target_velocity -= self.velocity;
		}

		target_velocity
	}

    pub fn flock(&mut self, other_boids: &Vec<Boid>)
    {
        let alignment = self.align(other_boids);
        self.acceleration = alignment;
    }
}

impl Nannou for Boid
{
	fn update(&mut self)
	{
		self.position += self.velocity;
		self.velocity += self.acceleration;
	}

	fn display(&self, draw: &nannou::Draw)
	{
		draw.ellipse()
			.x_y(self.position.x, self.position.y)
			.radius(self.radius)
			.color(self.color);
	}
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

pub struct BoidBuilder
{
	position: Option<Vec2>,
	velocity: Option<Vec2>,
	acceleration: Option<Vec2>,
	radius: Option<f32>,
	color: Option<Srgb<f32>>,
}

impl BoidBuilder
{
	pub fn position(&mut self, position: Vec2) -> &mut Self
	{
		self.position = Some(position);
		self
	}

	pub fn velocity(&mut self, velocity: Vec2) -> &mut Self
	{
		self.velocity = Some(velocity);
		self
	}

	pub fn acceleration(&mut self, acceleration: Vec2) -> &mut Self
	{
		self.acceleration = Some(acceleration);
		self
	}

	pub fn radius(&mut self, radius: f32) -> &mut Self
	{
		self.radius = Some(radius);
		self
	}

	pub fn color(&mut self, color: Srgb<f32>) -> &mut Self
	{
		self.color = Some(color);
		self
	}

	pub fn build(&mut self) -> Boid
	{
		Boid {
			position: self
				.position
				.unwrap_or_default(),
			velocity: self
				.velocity
				.unwrap_or_default(),
			acceleration: self
				.acceleration
				.unwrap_or_default(),
			radius: self
				.radius
				.unwrap_or_default(),
			color: self
				.color
				.unwrap_or_else(|| Srgb::<f32>::new(0.7, 0.2, 0.6)),
		}
	}
}

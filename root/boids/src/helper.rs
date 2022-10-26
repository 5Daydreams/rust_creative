use nannou::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Boid
{
	position: Vec2,
	velocity: Vec2,
	acceleration: Vec2,
	body_radius: f32,
	perception_radius: f32,
	max_speed: f32,
	steering_factor: f32,
	cohesion_factor: f32,
	separation_factor: f32,
	color: Srgb<f32>,
	window_size: [u32; 2],
}

pub trait Nannou
{
	fn update(&mut self);
	fn display(&self, draw: &nannou::Draw);
}

#[allow(clippy::new_ret_no_self)]
impl Boid
{
	pub fn new() -> BoidBuilder
	{
		BoidBuilder {
			position: None,
			velocity: None,
			acceleration: None,
			body_radius: None,
			perception_radius: None,
			max_speed: None,
			steering_factor: None,
			cohesion_factor: None,
			separation_factor: None,
			color: None,
			window_size: None,
		}
	}

	fn steer(&mut self, other_boids: &Vec<Boid>) -> Vec2
	{
		let mut alignment_steering: Vec2 = Vec2::new(0., 0.);
		let mut cohesion_steering: Vec2 = Vec2::new(0., 0.);
		let mut separation_steering: Vec2 = Vec2::new(0., 0.);
		let mut neighbour_count: i32 = 0;

		for other in other_boids
		{
			let dist = Vec2::distance(self.position, other.position);
			if dist < self.perception_radius && other != self
			{
				let separation_calc = self.position - other.position;
				separation_steering += separation_calc / (dist * dist);
				cohesion_steering += other.position;
				alignment_steering += other.velocity;
				neighbour_count += 1;
			}
		}

		if neighbour_count > 0
		{
			separation_steering /= neighbour_count as f32;
			separation_steering = separation_steering.normalize() * self.max_speed;
			separation_steering -= self.velocity;
			separation_steering = separation_steering.clamp_length_max(self.separation_factor);

			cohesion_steering /= neighbour_count as f32;
			cohesion_steering -= self.position;
			cohesion_steering = cohesion_steering.normalize() * self.max_speed;
			cohesion_steering -= self.velocity;
			cohesion_steering = cohesion_steering.clamp_length_max(self.cohesion_factor);

			alignment_steering /= neighbour_count as f32;
			alignment_steering = alignment_steering.normalize() * self.max_speed;
			alignment_steering -= self.velocity;
			alignment_steering = alignment_steering.clamp_length_max(self.steering_factor);
		}

		separation_steering + cohesion_steering + alignment_steering
	}

	pub fn flock(&mut self, other_boids: &Vec<Boid>)
	{
		let steer_value = self.steer(other_boids);
		self.acceleration += steer_value;
	}
}

impl Nannou for Boid
{
	fn update(&mut self)
	{
		if self.position.x < -(self.window_size[0] as f32) / 2.
		{
			self.position.x += self.window_size[0] as f32;
		}
		else if self.position.x > (self.window_size[0] as f32) / 2.
		{
			self.position.x -= self.window_size[0] as f32;
		}

		if self.position.y < -(self.window_size[1] as f32) / 2.
		{
			self.position.y += self.window_size[1] as f32;
		}
		else if self.position.y > (self.window_size[1] as f32) / 2.
		{
			self.position.y -= self.window_size[1] as f32;
		}

		self.position += self.velocity;
		self.velocity += self.acceleration;

		self.velocity = self
			.velocity
			.clamp_length_max(self.max_speed);

		self.acceleration *= 0.;
	}

	fn display(&self, draw: &nannou::Draw)
	{
		draw.ellipse()
			.x_y(self.position.x, self.position.y)
			.radius(self.body_radius)
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
			body_radius: 5.,
			perception_radius: 20_f32,
			max_speed: 20.,
			steering_factor: 0.01,
			cohesion_factor: 0.01,
			separation_factor: 0.01,
			color: Srgb::<f32>::new(0.7, 0.2, 0.6),
			window_size: [800, 600],
		}
	}
}

pub struct BoidBuilder
{
	position: Option<Vec2>,
	velocity: Option<Vec2>,
	acceleration: Option<Vec2>,
	body_radius: Option<f32>,
	perception_radius: Option<f32>,
	max_speed: Option<f32>,
	steering_factor: Option<f32>,
	cohesion_factor: Option<f32>,
	separation_factor: Option<f32>,
	color: Option<Srgb<f32>>,
	window_size: Option<[u32; 2]>,
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

	pub fn body_radius(&mut self, body_radius: f32) -> &mut Self
	{
		self.body_radius = Some(body_radius);
		self
	}

	pub fn max_speed(&mut self, max_speed: f32) -> &mut Self
	{
		self.max_speed = Some(max_speed);
		self
	}

	pub fn perception_radius(&mut self, perception_radius: f32) -> &mut Self
	{
		self.perception_radius = Some(perception_radius);
		self
	}

	pub fn steering_factor(&mut self, steering_factor: f32) -> &mut Self
	{
		self.steering_factor = Some(steering_factor);
		self
	}

	pub fn cohesion_factor(&mut self, cohesion_factor: f32) -> &mut Self
	{
		self.cohesion_factor = Some(cohesion_factor);
		self
	}

	pub fn separation_factor(&mut self, separation_factor: f32) -> &mut Self
	{
		self.separation_factor = Some(separation_factor);
		self
	}

	pub fn color(&mut self, color: Srgb<f32>) -> &mut Self
	{
		self.color = Some(color);
		self
	}

	pub fn window_size(&mut self, window_size: [u32; 2]) -> &mut Self
	{
		self.window_size = Some(window_size);
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
			body_radius: self
				.body_radius
				.unwrap_or_default(),
			perception_radius: self
				.perception_radius
				.unwrap_or_default(),
			max_speed: self
				.max_speed
				.unwrap_or_default(),
			steering_factor: self
				.steering_factor
				.unwrap_or_default(),
			cohesion_factor: self
				.cohesion_factor
				.unwrap_or_default(),
			separation_factor: self
				.separation_factor
				.unwrap_or_default(),
			color: self
				.color
				.unwrap_or_else(|| Srgb::<f32>::new(0.7, 0.2, 0.6)),
			window_size: self
				.window_size
				.unwrap_or([800, 600]),
		}
	}
}

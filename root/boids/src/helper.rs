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
	max_steering: f32,
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
			max_steering: None,
			color: None,
			window_size: None,
		}
	}

	fn steer(&mut self, other_boids: &Vec<Boid>) -> Vec2
	{
		let mut target_velocity: Vec2 = Vec2::new(0., 0.);
		let mut target_pos_midpoint: Vec2 = Vec2::new(0., 0.);
		let mut neighbour_count: i32 = 0;

		for other in other_boids
		{
			let dist = Vec2::distance(self.position, other.position);
			if dist < self.perception_radius && other != self
			{
				target_pos_midpoint += other.position;
				target_velocity += other.velocity;
				neighbour_count += 1;
			}
		}

		if neighbour_count > 0
		{
			target_pos_midpoint /= neighbour_count as f32;
			target_pos_midpoint -= self.position;
			target_pos_midpoint = target_pos_midpoint.normalize() * self.max_speed;
			target_pos_midpoint -= self.velocity;
			target_pos_midpoint.clamp_length_max(self.max_steering);

			target_velocity /= neighbour_count as f32;
			target_velocity = target_velocity.normalize() * self.max_speed;
			target_velocity -= self.velocity;
			target_velocity.clamp_length_max(self.max_steering);
		}

		target_pos_midpoint // + target_velocity
	}

	pub fn flock(&mut self, other_boids: &Vec<Boid>)
	{
		let steer_value = self.steer(other_boids);
		self.acceleration = steer_value;
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
			max_steering: 2.,
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
	max_steering: Option<f32>,
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

	pub fn max_steering(&mut self, max_steering: f32) -> &mut Self
	{
		self.max_steering = Some(max_steering);
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
			max_steering: self
				.max_steering
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

use nannou::prelude::*;
// nannou provides this thing called a prelude to help with
// handling reusage of code from other scripts;
use rand::prelude::*;

// Trait for display & update() methods
// ------------------------------------------------------------------------------------------
pub trait Nannou
{
	fn display(&self, draw: &nannou::Draw);
	fn update(&mut self);
}

// Vec 2d struct
// ------------------------------------------------------------------------------------
pub struct Vec2<T>
{
	pub x: T,
	pub y: T,
}

type PointInt = Vec2<u32>;

impl PointInt
{
	pub fn new(x: u32, y: u32) -> Self { Self { x, y } }
}

type Point = Vec2<f32>;

impl Point
{
	pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

// Dot Struct, a circle with a position in the screen
// ----------------------------------------------------------------------
pub struct Dot
{
	pub color: Srgba<u8>,
	pub origin: Point,
	pub radius: f32,
	pub max_radius: f32,
	pub growth_rate: f32,
}

impl Dot
{
	pub fn new() -> Self { Self::default() }
}

impl Nannou for Dot
{
	fn display(&self, draw: &nannou::Draw)
	{
		draw.ellipse()
			.w(self.radius)
			.h(self.radius)
			.x_y(self.origin.x, self.origin.y)
			.color(self.color);
	}
	fn update(&mut self)
	{
		if self.radius < self.max_radius
		{
			self.radius += self.growth_rate;
		}
		else
		{
			let mut rng: ThreadRng = rand::thread_rng();
			self.radius = 0.0;

			let x: f32 = rng.gen_range(-250.0 .. 250.0);
			let y: f32 = rng.gen_range(-250.0 .. 250.0);

			let new_pos: Point = Point::new(x, y);
			self.origin = Point::new(new_pos.x, new_pos.y);

			let mut rng: ThreadRng = rand::thread_rng();
			let growth_val: f32 = rng.gen_range(0.5 .. 3.0);

			self.growth_rate = growth_val;
		}
	}
}

impl Default for Dot
{
	fn default() -> Self
	{
		Self {
			color: Srgba::<u8>::new(15, 80, 180, 105),
			origin: Point::new(0.0, 0.0),
			radius: 10.0,
			max_radius: 200.0,
			growth_rate: 1.0,
		}
	}
}

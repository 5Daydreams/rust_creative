use nannou::prelude::*;

pub struct Vector2<T>
{
	pub x: T,
	pub y: T,
}

pub type Vec2Int = Vector2<u32>;

impl Vec2Int
{
	pub fn new(x: u32, y: u32) -> Self { Self { x: x, y: y } }
}

pub struct Model
{
	pub bg_color: Srgb<f32>,
	pub window_size: Vec2Int,
}

impl Model
{
	pub fn new(color: Srgb<f32>, size: Vec2Int) -> Self
	{
		Self {
			bg_color: color,
			window_size: size,
		}
	}
}

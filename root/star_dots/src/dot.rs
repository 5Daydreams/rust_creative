use nannou::prelude::*;

use crate::{
	constants::*,
	trait_nannou::Nannou,
	vec_extensions::Perspective2D,
};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Default, Clone, PartialEq)]
pub struct Dot
{
	pub pos: Vec3,

	#[builder(default = 0)]
	pub neighbours: u32,

	#[builder(default = 2.5)]
	pub radius: f32,

	#[builder(default = Srgb::<f32>::new(0.8,0.8,0.8))]
	pub color: Srgb<f32>,
}

impl Nannou for Dot
{
	fn display(&self, draw: &nannou::Draw)
	{
		let window_size = [DEFAULT_WINDOW_SIZE.0, DEFAULT_WINDOW_SIZE.1];
		let offset = OFFSET_VEC;

		let display_pos = self.pos.project_into_2d(
			offset,
			window_size,
			FOV,
			CLIPPING_PLANES.0,
			CLIPPING_PLANES.1,
		);

		let converted_z = 1. - ((self.pos.z- CLIPPING_PLANES.0)/ (CLIPPING_PLANES.1- CLIPPING_PLANES.0));
		let final_radius = self.radius * converted_z;

		draw.ellipse()
			.x_y(display_pos.x, display_pos.y)
			.radius(final_radius)
			.color(self.color);
	}
}

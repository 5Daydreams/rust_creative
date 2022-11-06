use nannou::prelude::*;

use crate::{
	constants::*,
	trait_nannou::Nannou,
	vec_extensions::Perspective2D,
};
use typed_builder::TypedBuilder;

const EDGE_STRENGTH: f32 = 50.;

#[derive(TypedBuilder, Default, Clone)]
pub struct Edge
{
	pub start: Vec2,
	pub end: Vec2,

	#[builder(default = Srgb::<f32>::new(0.05,0.1,0.3))]
	pub color: Srgb<f32>,
}

impl Nannou for Edge
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

		let converted_z =
			1. - ((self.pos.z - CLIPPING_PLANES.0) / (CLIPPING_PLANES.1 - CLIPPING_PLANES.0));
		let final_radius = self.radius * converted_z;

		draw.ellipse()
			.x_y(display_pos.x, display_pos.y)
			.radius(final_radius)
			.color(self.color);

		let point_a = self.point_list[(i) % list_size].pos;
		let point_b = self.point_list[(i + list_size / 2) % list_size].pos;
		let point_c = self.point_list[(i + list_size / 3) % list_size].pos;

		let display_pos_a = point_a.project_into_2d(
			offset,
			window_size,
			FOV,
			CLIPPING_PLANES.0,
			CLIPPING_PLANES.1,
		);

		let display_pos_b = point_b.project_into_2d(
			offset,
			window_size,
			FOV,
			CLIPPING_PLANES.0,
			CLIPPING_PLANES.1,
		);

		let display_pos_c = point_c.project_into_2d(
			offset,
			window_size,
			FOV,
			CLIPPING_PLANES.0,
			CLIPPING_PLANES.1,
		);

		//  let funny_number = rand::Rng::gen_range(&mut rand::thread_rng(), 0.0 .. 1.0);

		let dist = (end - start).magnitude;

		let funny_number = EDGE_STRENGTH;

		let default_line_color =
			Srgb::<f32>::new(0.03, 0.06 + 0.08 * funny_number, 0.15 + 0.1 * funny_number);

		draw.line()
			.caps_round()
			.start(display_pos_a)
			.end(display_pos_b)
			.color(default_line_color);

		draw.line()
			.caps_round()
			.start(display_pos_b)
			.end(display_pos_c)
			.color(default_line_color);
	}
}

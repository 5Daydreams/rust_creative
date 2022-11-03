use constants::*;
use dot::Dot;
use nannou::prelude::*;

mod constants;
mod dot;
mod range_cube;
mod trait_nannou;
mod vec_extensions;

use range_cube::*;
use trait_nannou::Nannou;
use vec_extensions::{
	Perspective2D,
	RotateMatrixBaked,
};

pub struct Model
{
	pub curr_time: f32,
	pub prev_time: f32,
	pub delta_time: f32,
	pub point_list: Vec<Dot>,
	pub bounding_box: RangeCube,
	pub window_size: (u32, u32),
}

impl Model
{
	pub fn new() -> Self
	{
		use constants::*;
		let box_ranges: RangeCube = RangeCube::new(SPAWN_RANGE_X, SPAWN_RANGE_Y, SPAWN_RANGE_Z);

		Self {
			curr_time: 0.,
			prev_time: 0.,
			delta_time: 0.16,
			point_list: Vec::new(),
			bounding_box: box_ranges,
			window_size: DEFAULT_WINDOW_SIZE,
		}
	}

	pub fn update(&mut self)
	{
		for point in &mut self.point_list
		{
			point.pos = point
				.pos
				.rotate_y(self.delta_time / 2.);
		}
	}
}

impl Default for Model
{
	fn default() -> Self { Self::new() }
}

impl Nannou for Model
{
	fn display(&self, draw: &nannou::Draw)
	{
		let window_size = [DEFAULT_WINDOW_SIZE.0, DEFAULT_WINDOW_SIZE.1];
		let offset = OFFSET_VEC;

		let list_size = self.point_list.len();

		for i in list_size / 3 .. list_size
		{
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

			let funny_number = i as f32 / list_size as f32;

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

		for point in &self.point_list
		{
			point.display(draw);
		}
	}
}

pub fn model(_app: &App) -> Model
{
	let mut model: Model = Model::new();

	_app.new_window()
		.size(model.window_size.0, model.window_size.1)
		.title("Gradient Dots :v")
		.view(view)
		.build()
		.unwrap();

	for _ in 0 .. POINT_COUNT
	{
		let random_pos = model
			.bounding_box
			.get_random_vec3();

		let point = Dot::builder()
			.pos(random_pos)
			.build();

		model
			.point_list
			.push(point);
	}

	// model
	// 	.point_list
	// 	.sort_by(|a, b| {
	// 		a.pos
	// 			.y
	// 			.total_cmp(&b.pos.y)
	// 	});

	model
}

pub fn update(_app: &App, model: &mut Model, _update: Update)
{
	model.curr_time = _app.time;
	model.delta_time = model.curr_time - model.prev_time;

	model.update();
	model.prev_time = model.curr_time;
}

pub fn view(app: &App, model: &Model, frame: Frame)
{
	let draw = app.draw();

	let bg_color: Srgb<f32> = Srgb::<f32>::new(0.01, 0.05, 0.1);

	draw.background()
		.color(bg_color);

	model.display(&draw);

	draw.to_frame(app, &frame)
		.unwrap();
}

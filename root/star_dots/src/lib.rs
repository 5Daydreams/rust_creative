use constants::*;
use dot::Dot;
use edge::Edge;
use nannou::{
	geom::point,
	prelude::*,
};

mod constants;
mod dot;
mod edge;
mod range_cube;
mod trait_nannou;
mod vec_extensions;

use range_cube::*;
use trait_nannou::Nannou;
use vec_extensions::RotateMatrixBaked;

pub struct Model
{
	pub curr_time: f32,
	pub prev_time: f32,
	pub delta_time: f32,
	pub point_list: Vec<Dot>,
	pub edge_list: Vec<Edge>,
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
			edge_list: Vec::new(),
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
		for point in &self.point_list
		{
			point.display(draw);
		}

		for edge in &self.edge_list
		{
			edge.display(draw);
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

	let list_copy = &model.point_list;

	for point_a in &model.point_list
	{
		if point_a.neighbours >= 3
		{
			continue;
		}

		let mut closest_list: Vec<(&Dot, f32)> = Vec::new();

		for point_b in list_copy
		{
			if &point_a == &point_b
			{
				continue;
			}

			let dist = point_a
				.pos
				.distance(point_b.pos);

			let candidate = (point_b, dist);

			for closest in closest_list
			{
				if closest.1 > candidate.1
				{
					closest_list.first()
					closest_list.remove(index)
					closest_list.push(candidate);
				}
			}

		}

		// this should NOT be necessary, but oh well
		assert_eq!(point_a.neighbours, 3);
	}

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

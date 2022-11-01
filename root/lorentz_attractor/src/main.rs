use std::ops::Range;

use nannou::prelude::*;

mod helper;
use helper::*;

mod vec_extensions;
use rand::Rng;
use vec_extensions::Randomize;

fn main()
{
	nannou::app(model)
		.update(update)
		.run();
}

struct Model
{
	lor_points: Vec<LorentzPoint>,
	window_size: [u32; 2],
	time: Time,
}

impl Nannou for Model
{
	fn update(&mut self)
	{
		let delta: f32 = self.time.delta_time;
		for lor_point in &mut self.lor_points
		{
			lor_point.refresh_time(delta);
			lor_point.update();
		}
	}

	fn display(&self, draw: &nannou::Draw)
	{
		for lor_point in &self.lor_points
		{
			lor_point.display(draw);
		}
	}
}

fn model(_app: &App) -> Model
{
	let mut model: Model = Model {
		lor_points: Vec::new(),
		window_size: [1200, 800],
		time: Time {
			curr_time: _app.time,
			prev_time: _app.time,
			delta_time: 1. / 60.,
		},
	};

	let z_near: f32 = 1.0;
	let z_far: f32 = 1000.0;
	let field_of_view: f32 = PI * 0.3;
	let constants_vec: Vec3 = Vec3::new(12., 1.5, 25.);
	let curve_count: u32 = 1500;

	let mut rand_vec: Vec3 = Vec3::new(0., 0., 0.);

	for _ in 0 .. curve_count
	{
		let box_range: f32 = 5.0;

		let x_range_pos: Range<f32> = -box_range .. box_range;
		let y_range_pos: Range<f32> = -box_range .. box_range;
		let z_range_pos: Range<f32> = -box_range .. box_range;

		rand_vec.randomize(x_range_pos, y_range_pos, z_range_pos);

		let r_range: Range<f32> = 0.25 .. 0.40;
		let g_range: Range<f32> = 0.01 .. 0.05;
		let b_range: Range<f32> = 0.7 .. 0.85;

		let r: f32 = rand::thread_rng().gen_range(r_range);
		let g: f32 = rand::thread_rng().gen_range(g_range);
		let b: f32 = rand::thread_rng().gen_range(b_range);

		let point_color: Srgb<f32> = Srgb::<f32>::new(r, g, b);

		model
			.lor_points
			.push(LorentzPoint {
				near_plane: z_near,
				far_plane: z_far,
				fov_radians: field_of_view,
				window_size: model.window_size,
				delta_time: model.time.delta_time,
				curr_time: 0.,
				lorentz_constants: constants_vec,
				prev_point: rand_vec,
				curr_point: rand_vec,
				color: point_color,
			});
	}

	_app.new_window()
		.size(model.window_size[0], model.window_size[1])
		.view(view)
		.build()
		.unwrap();

	model
}

fn update(_app: &App, model: &mut Model, _update: Update)
{
	model.time.delta_time = model.time.curr_time - model.time.prev_time;
	model.time.prev_time = model.time.curr_time;
	model.update();
	model.time.curr_time = _app.time;
}

fn view(_app: &App, model: &Model, frame: Frame)
{
	let draw: Draw = _app.draw();

	let bg_color: Srgb<f32> = Srgb::<f32>::new(0.02, 0.05, 0.1);

	draw.background()
		.color(bg_color);

	model.display(&draw);

	draw.to_frame(_app, &frame)
		.unwrap();
}

mod helper_structs;

use helper_structs::*;
use nannou::draw::background::new;
use nannou::noise::{
	NoiseFn,
	Perlin,
};
use nannou::prelude::*;

fn main() { nannou::app(model).run(); }

fn model(_app: &App) -> Model
{
	let color_value: Srgb<f32> = Srgb::<f32>::new(0.05, 0.1, 0.1);
	let w_size = Vec2Int::new(800, 600);

	let model: Model = Model::new(color_value, w_size);

	_app.new_window()
		.size(model.window_size.x, model.window_size.y)
		.view(view)
		.build()
		.unwrap();

	return model;
}

fn view(_app: &App, _model: &Model, frame: Frame)
{
	let draw: Draw = _app.draw();

	draw.background()
		.color(_model.bg_color);

	const SPEED: f32 = 2.0;
	let t: f32 = _app.time * SPEED;

	let width: f32 = 350.;
	let height: f32 = 150.;

	let sampling_scale: f32 = 5.0;

	let step_size: usize = 5;
	let element_count: usize = ((width * 2.0) as usize) / step_size;

	let mut points: Vec<Vec2> = Vec::new();

	let start_pos: f32 = -width;

	let sample_value = (start_pos * sampling_scale).cos();
	let point_pos: Vec2 = Vec2::new(start_pos, height * sample_value);

	points.push(point_pos);

	for i in 1 .. element_count
	{
		// match i % 2 {
        //     1 => // do top line
        //     _ => // do bottom line
        // }

		let sampling_pos: f32 = 2.0 * width * (i as f32 / element_count as f32) + start_pos;

		let sample_value: f32 = (sampling_pos * sampling_scale).cos();
		let perlin_value: f32 = height * sample_value;

		let offset = (1.0 - 2.0 * (i % 2) as f32) * height * 0.5;

		let curr_vec = Vec2::new(sampling_pos, perlin_value - offset);

		points.push(curr_vec);

		draw.line()
			.start(points[i - 1])
			.end(points[i])
			.caps_round()
			.stroke_weight(3.0)
			.color(Srgb::<f32>::new(0.9, 0.9, 0.9));
	}

	draw.to_frame(_app, &frame)
		.unwrap();
}

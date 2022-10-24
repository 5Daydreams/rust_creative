mod helper_structs;

use helper_structs::*;
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

	const SPEED: f64 = 2.0;
	let t: f64 = (_app.time as f64) * SPEED;

	let horizontal: f32 = 300.;
	let vertical: f32 = 150.;

	let range: std::ops::RangeInclusive<i32> = -horizontal as i32 ..= horizontal as i32;

	let step_size: usize = 30;
	let perlin: Perlin = Perlin::new();
	let noise_scale: f64 = 5.0;
	let amplitude_scale = 0.7;
	let amplitude_offset = 1.0;

	let mut _last_end_pos: Option<Vec2>;

	for value in range.step_by(step_size)
	{
		let perlin_sample_pos: f64 = (value as f64 / (horizontal as f64)) * noise_scale;

		let point_pos: [f64; 2] = [t + perlin_sample_pos * 1.1, perlin_sample_pos * 0.2];
		let perlin_value_start: f32 = perlin.get(point_pos) as f32;
		let start_x_offset: f32 = perlin_value_start * (step_size as f32);
		let start_pos: Vec2 = Vec2::new(
			(value as f32) + start_x_offset,
			vertical * (perlin_value_start * amplitude_scale - amplitude_offset),
		);

		let point_pos: [f64; 2] = [t + perlin_sample_pos * 0.2, perlin_sample_pos * 1.1];
		let perlin_value_end: f32 = perlin.get(point_pos) as f32;
		let end_x_offset: f32 = perlin_value_end * (step_size as f32);
		let end_pos: Vec2 = Vec2::new(
			(value as f32) - end_x_offset,
			vertical * (perlin_value_end * amplitude_scale + amplitude_offset),
		);

		let perlin_value_delta: f32 =
			((end_pos.y) - (start_pos.y)) / (2.5 * (amplitude_scale + amplitude_offset) * vertical);

		draw.line()
			.start(start_pos)
			.end(end_pos)
			.caps_round()
			.stroke_weight(3.0)
			.color(Srgb::<f32>::new(
				perlin_value_delta,
				perlin_value_delta,
				perlin_value_delta,
			));

		// if last_end_pos.is_some()
		// {
		// 	draw.line()
		// 		.start(start_pos)
		// 		.end(last_end_pos.unwrap())
		// 		.caps_round()
		// 		.stroke_weight(3.0)
		// 		.color(Srgb::<f32>::new(
		// 			perlin_value_delta,
		// 			perlin_value_delta,
		// 			perlin_value_delta,
		// 		));
		// }

		// last_end_pos = Some(end_pos);
	}

	draw.to_frame(_app, &frame)
		.unwrap();
}

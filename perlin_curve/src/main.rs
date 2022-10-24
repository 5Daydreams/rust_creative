mod helper_structs;

use helper_structs::*;
// use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
	nannou::app(model).run();
}

fn model(_app: &App) -> Model {
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

fn view(_app: &App, _model: &Model, frame: Frame) {
	let draw: Draw = _app.draw();

	draw.background()
		.color(_model.bg_color);

	const SPEED: f32 = 150.0;
	let t: f32 = _app.time * SPEED;

	let width: f32 = 350.;
	let height: f32 = 150.;

	let sampling_scale: f32 = 3.0;
	let sample_offset: f32 = 1.7;
	let skewness: f32 = 0.25;

	let step_size: usize = 7;
	let element_count: usize = ((width * 2.0) as usize) / step_size;
	let mut points: Vec<Vec2> = Vec::new();

	let start_x_value: f32 = -width;

	let sample_value: f32 = ((start_x_value + t) * sampling_scale) / width;

	let start_sample_result =
		helper_structs::skewed_cos(sample_value + sample_offset * 0.5 - PI * 0.5, skewness) * 0.5
			+ 0.5;

	let point_pos: Vec2 = Vec2::new(
		start_x_value,
		height * (sample_value) * (0.5 + 0.5 * start_sample_result),
	);

	points.push(point_pos);

	for i in 1..element_count {
		let x_value: f32 = 2.0 * width * (i as f32 / element_count as f32) + start_x_value;

		let sample_value: f32 = ((x_value + t) * sampling_scale) / width;

		let sample_result: f32 = match i % 2 {
			| 0 => helper_structs::skewed_cos(sample_value, skewness), // do top line
			| _ => helper_structs::skewed_cos(sample_value + sample_offset, skewness), // do bottom line
		};

		let y_value: f32 = height * sample_result;

		let normalized_sample_result =
			helper_structs::skewed_cos(sample_value + sample_offset * 0.5 - PI * 0.5, skewness)
				* 0.5 + 0.5;

		let curr_vec = Vec2::new(x_value, y_value * (0.5 + 0.5 * normalized_sample_result));

		points.push(curr_vec);

		draw.line()
			.start(points[i - 1])
			.end(points[i])
			.caps_round()
			.stroke_weight(2.0 + 1.5 * normalized_sample_result)
			.color(Srgb::<f32>::new(
				normalized_sample_result,
				normalized_sample_result,
				normalized_sample_result,
			));
	}

	draw.to_frame(_app, &frame)
		.unwrap();
}

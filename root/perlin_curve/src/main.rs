mod helper_structs;

use helper_structs::*;
// use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() { nannou::app(model).run(); }

fn model(_app: &App) -> Model
{
	let color_value: Srgb<f32> = Srgb::<f32>::new(0.05, 0.1, 0.1);
	let w_size: helper_structs::Vector2<u32> = Vec2Int::new(800, 600);

	let model: Model = Model::new(color_value, w_size);

	_app.new_window()
		.size(model.window_size.x, model.window_size.y)
		.view(view)
		.build()
		.unwrap();

	model
}

fn view(_app: &App, _model: &Model, frame: Frame)
{
	let draw: Draw = _app.draw();

	draw.background()
		.color(_model.bg_color);

	const SCROLL_SPEED: f32 = 150.0;
	let t: f32 = _app.time * SCROLL_SPEED;

	const WIDTH: f32 = 350.;
	const HEIGHT: f32 = 200.;

	const SAMPLING_SCALE: f32 = 3.0;
	const SAMPLE_OFFSET: f32 = 1.7;
	const SKEWNESS: f32 = 0.25;
	const BASE_LERP_VALUE: f32 = 0.4;
	const MIDPOINT_LERP_VALUE: f32 = 0.4;
	const MIDPOINT_OFFSET: f32 = SAMPLE_OFFSET * 0.5 - PI * 0.5;

	const STEP_SIZE: usize = 7;
	const ELEMENT_COUNT: usize = ((WIDTH * 2.0) as usize) / STEP_SIZE;
	let mut points: Vec<Vec2> = Vec::new();

	let start_x_value: f32 = -WIDTH;

	let sample_value: f32 = ((start_x_value + t) * SAMPLING_SCALE) / WIDTH;
	let sample_result: f32 = helper_structs::skewed_cos(sample_value, SKEWNESS);

	let y_value: f32 = HEIGHT * sample_result;

	let normalized_sample_result: f32 =
		helper_structs::skewed_cos(sample_value + MIDPOINT_OFFSET, SKEWNESS) * 0.5 + 0.5;

	let curr_vec: Vec2 = Vec2::new(
		start_x_value,
		y_value * (BASE_LERP_VALUE + MIDPOINT_LERP_VALUE * normalized_sample_result),
	);

	points.push(curr_vec);

	for i in 1 .. ELEMENT_COUNT
	{
		let x_value: f32 = 2.0 * WIDTH * (i as f32 / ELEMENT_COUNT as f32) + start_x_value;

		let sample_value: f32 = ((x_value + t) * SAMPLING_SCALE) / WIDTH;

		let sample_result: f32 = match i % 2
		{
			| 0 => helper_structs::skewed_cos(sample_value, SKEWNESS), // do top line
			| _ => helper_structs::skewed_cos(sample_value + SAMPLE_OFFSET, SKEWNESS), // do bottom line
		};

		let y_value: f32 = HEIGHT * sample_result;

		let normalized_sample_result: f32 =
			helper_structs::skewed_cos(sample_value + MIDPOINT_OFFSET, SKEWNESS) * 0.5 + 0.5;

		let curr_vec: Vec2 = Vec2::new(
			x_value,
			y_value * (BASE_LERP_VALUE + MIDPOINT_LERP_VALUE * normalized_sample_result),
		);

		points.push(curr_vec);

		draw.line()
			.start(points[i - 1])
			.end(points[i])
			.caps_round()
			.stroke_weight(1.5 + 2.0 * normalized_sample_result)
			.color(Srgb::<f32>::new(
				normalized_sample_result,
				normalized_sample_result,
				normalized_sample_result,
			));
	}

	draw.to_frame(_app, &frame)
		.unwrap();
}

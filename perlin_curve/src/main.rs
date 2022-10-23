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

	let t: f32 = _app.time;

	draw.background()
		.color(_model.bg_color);

	let start_pos: Vec2 = Vec2::new(0.0, 0.0);

	let radius: f32 = 200.;
	let cos: f32 = t.cos();
	let sin: f32 = t.sin();

	let end_pos: Vec2 = Vec2::new(radius * cos, radius * sin);

	let thing: Perlin = Perlin::new();
	let point_pos: [f64; 2] = [start_pos.x as f64, start_pos.y as f64];
	thing.get(point_pos);

	draw.line()
		.start(start_pos)
		.end(end_pos)
		.color(Srgb::<f32>::new(1.0, 1.0, 1.0));

	draw.to_frame(_app, &frame)
		.unwrap();
}

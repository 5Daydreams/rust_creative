use nannou::prelude::*;

fn main()
{
	nannou::app(model)
		.update(update)
		.run();
}

struct Model {}

fn model(_app: &App) -> Model { Model {} }

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(_app: &App, model: &Model, frame: Frame)
{
	let draw = _app.draw();

	_app.new_window()
		.size(800, 600)
		.view(view)
		.build()
		.unwrap();

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

	draw.to_frame(&_app, &frame)
		.unwrap();
}

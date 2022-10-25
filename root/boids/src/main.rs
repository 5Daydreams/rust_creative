use nannou::prelude::*;

mod helper;
use helper::*;

fn main()
{
	nannou::app(model)
		.update(update)
		.run();
}

struct Model {}

fn model(_app: &App) -> Model
{
	let window_size: [u32; 2] = [800, 600];

	_app.new_window()
		.size(window_size[0], window_size[1])
		.view(view)
		.build()
		.unwrap();

	Model {}
}

fn update(_app: &App, model: &mut Model, _update: Update) {}

fn view(_app: &App, model: &Model, frame: Frame)
{
	let draw = _app.draw();

	let line_start = Vec2::new(-200.0, 100.0);
	let line_end = Vec2::new(200.0, -100.0);

	draw.ellipse()
		.x_y(0., 0.)
		.radius(5.)
		.color(Srgb::<f32>::new(0.7, 0.2, 0.5));

	draw.line()
		.start(line_start)
		.end(line_end)
		.caps_round()
		.stroke_weight(1.5)
		.color(Srgb::<f32>::new(0.7, 0.7, 0.7));

	draw.to_frame(_app, &frame)
		.unwrap();
}

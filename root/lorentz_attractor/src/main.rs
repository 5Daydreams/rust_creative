use nannou::prelude::*;

mod helper;

use helper::*;

fn main()
{
	nannou::app(model)
		.update(update)
		.run();
}

struct Model
{
	window_size: [u32; 2],
}

impl Nannou for Model
{
	fn display(&self, draw: &nannou::Draw) {}

	fn update(&mut self) {}
}

fn model(_app: &App) -> Model
{
	let model = Model {
		window_size: [1200, 800],
	};

	_app.new_window()
		.size(model.window_size[0], model.window_size[1])
		.view(view)
		.build()
		.unwrap();

	model
}

fn update(_app: &App, model: &mut Model, _update: Update) { model.update(); }

fn view(_app: &App, model: &Model, frame: Frame)
{
	let draw = _app.draw();

	draw.background()
		.color(Srgb::<f32>::new(0.05, 0.1, 0.15));

	model.display(&draw);

	draw.to_frame(_app, &frame)
		.unwrap();
}

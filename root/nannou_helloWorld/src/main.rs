use nannou::prelude::*;
// nannou provides this thing called a prelude to help with
// handling reusage of code from other scripts;

mod nk_aux;

use nk_aux::Nannou;
use nk_aux::Vec2;

fn main()
{
	nannou::app(model)
		.update(update)
		.run();
}


struct Model
{
	bg_color: Srgba<u8>,
	window_size: Vec2<u32>,
	dot: nk_aux::Dot,
}

impl Default for Model
{
	fn default() -> Self
	{
		Self {
			bg_color: Srgba::<u8>::new(5,10,10,255),
            window_size: Vec2::<u32>::new(800, 600),
			dot: nk_aux::Dot::new(),
		}
	}
}

impl nk_aux::Nannou for Model
{
	fn display(&self, draw: &nannou::Draw)
	{
		draw.background()
			.color(self.bg_color);
		self.dot
			.display(draw);
	}

	fn update(&mut self) { self.dot.update(); }
}

fn model(_app: &App) -> Model
{
	let model: Model = Model::default();

	_app.new_window()
		.size(model.window_size.x, model.window_size.y)
		.view(view)
		.build()
		.unwrap();

	return model;
}

fn update(_app: &App, model: &mut Model, _update: Update) { model.update(); }

fn view(app: &App, model: &Model, frame: Frame)
{
	let draw = app.draw();

    model.display(&draw);

    draw.to_frame(&app, &frame)
		.unwrap();
}

use nannou::prelude::*;
// nannou provides this thing called a prelude to help with
// handling reusage of code from other scripts;
use rand::prelude::*;

use structopt::StructOpt;

use lazy_static::lazy_static;

lazy_static! {
	pub static ref OPT: Opt = Opt::from_args();
}

/// A nannou demonstration application
#[derive(StructOpt, Debug)]
#[structopt(name = "nannou_dots")]
pub struct Opt
{
	/// Set dot growth rate
	#[structopt(short, long, default_value = "1.0")]
	rate: f32,
}

fn main()
{
	// sketches take no input/output, apps are more
	// complex nannou::sketch(view).run();
	nannou::app(model)
		.update(update)
		.run();
}

/// A coordinate pair - the (0,0) default is the center of the frame
#[derive(Debug, Default, Clone, Copy)]
struct Point
{
	x: f32,
	y: f32,
}

// Constructor method for the Point struct
impl Point
{
	fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

/// Things that can be drawn to the screen
trait Nannou
{
	fn display(&self, draw: &nannou::Draw);
	fn update(&mut self);
}

/// A circle to paint
#[derive(Debug, Clone, Copy)]
struct Dot
{
	color: Srgba<u8>,
	origin: Point,
	radius: f32,
	max_radius: f32,
	growth_rate: f32,
}

impl Dot
{
	fn new() -> Self { Self::default().set_growth_rate(OPT.rate) }
	fn set_growth_rate(mut self, rate: f32) -> Self
	{
		self.growth_rate = rate;
		return self;
	}
}

impl Nannou for Dot
{
	fn display(&self, draw: &nannou::Draw)
	{
		draw.ellipse()
			.w(self.radius)
			.h(self.radius)
			.x_y(self.origin.x, self.origin.y)
			.color(self.color);
	}
	fn update(&mut self)
	{
		if self.radius < self.max_radius
		{
			self.radius += self.growth_rate;
		}
		else
		{
			let mut rng: ThreadRng = rand::thread_rng();
			self.radius = 0.0;

			let x: f32 = rng.gen_range(-250.0 .. 250.0);
			let y: f32 = rng.gen_range(-250.0 .. 250.0);

			let new_pos: Point = Point::new(x, y);
			self.origin = Point::new(new_pos.x, new_pos.y);
		}
	}
}

impl Default for Dot
{
	fn default() -> Self
	{
		Self {
			color: Srgba::<u8>::new(15,80,180,105),
			origin: Point::default(),
			radius: 10.0,
			max_radius: 200.0,
			growth_rate: 1.0,
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
struct Vec2<T>
{
	x: T,
	y: T,
}

// Constructor method for the Point struct
impl Vec2<u32>
{
	fn new(x: u32, y: u32) -> Self { Self { x, y } }
}

/// The application state
#[derive(Debug)]
struct Model
{
	bg_color: Srgba<u8>,
	window_size: Vec2<u32>,
	dot: Dot,
}

impl Default for Model
{
	fn default() -> Self
	{
		Self {
			bg_color: Srgba::<u8>::new(150,140,120,255),
            window_size: Vec2::<u32>::new(800, 600),
			dot: Dot::new(),
		}
	}
}

impl Nannou for Model
{
	/// Show this model
	fn display(&self, draw: &nannou::Draw)
	{
		draw.background()
			.color(self.bg_color);
		self.dot
			.display(draw);
	}
	/// Update this model
	fn update(&mut self) { self.dot.update(); }
}

// Nannou interface
//

/// Nannou app model
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

/// Nannou app update
fn update(_app: &App, model: &mut Model, _update: Update) { model.update(); }

/// Nannou app view
fn view(app: &App, model: &Model, frame: Frame)
{
	let draw = app.draw();
	// Draw model
	model.display(&draw);
	// Render frame
	draw.to_frame(&app, &frame)
		.unwrap();
}

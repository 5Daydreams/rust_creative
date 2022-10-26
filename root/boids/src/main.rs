use nannou::prelude::*;

mod helper;
mod vec_extensions;

use helper::*;
use rand::{
	self,
	Rng,
};
use vec_extensions::Randomize;

fn main()
{
	nannou::app(model)
		.update(update)
		.run();
}

struct Model
{
	boid_count: u32,
	boid_list: Vec<Boid>,
	window_size: [u32; 2],
}

impl Nannou for Model
{
	fn display(&self, draw: &nannou::Draw)
	{
		for boid in &self.boid_list
		{
			boid.display(draw);
		}
	}

	fn update(&mut self)
	{
		let old_list = self
			.boid_list
			.clone();

		for boid in &mut self.boid_list
		{
			boid.flock(&old_list);
			boid.update();
		}
	}
}

fn model(_app: &App) -> Model
{
	let mut model = Model {
		boid_count: 150,
		boid_list: Vec::new(),
		window_size: [800, 600],
	};

	_app.new_window()
		.size(model.window_size[0], model.window_size[1])
		.view(view)
		.build()
		.unwrap();

	for _ in 0 .. model.boid_count
	{
		let mut rand_vec: Vec2 = Vec2::new(0., 0.);

		let x_range_pos = -350.0 .. 3500.;
		let y_range_pos = -200.0 .. 200.;

		rand_vec.randomize(x_range_pos, y_range_pos);
		let position: Vec2 = rand_vec;

		let x_range_vel = -2.0 .. 2.;
		let y_range_vel = -2.0 .. 2.;

		rand_vec.randomize(x_range_vel, y_range_vel);
		let velocity: Vec2 = rand_vec;

		let x_range_acc = -0.01 .. 0.01;
		let y_range_acc = -0.01 .. 0.01;

		rand_vec.randomize(x_range_acc, y_range_acc);
		let acceleration: Vec2 = rand_vec;

		let body_radius: f32 = rand::thread_rng().gen_range(3. .. 4.);
		let perception_radius: f32 = 50.;
		let max_speed: f32 = 1.;
		let max_steering: f32 = 0.00000000001;

		let r: f32 = rand::thread_rng().gen_range(0.1 .. 0.4);
		let g: f32 = rand::thread_rng().gen_range(0.15 .. 0.2);
		let b: f32 = rand::thread_rng().gen_range(0.65 .. 0.7);

		let color = Srgb::<f32>::new(r, g, b);

		let boid: Boid = Boid::new()
			.position(position)
			.velocity(velocity)
			.acceleration(acceleration)
			.body_radius(body_radius)
			.perception_radius(perception_radius)
			.max_speed(max_speed)
			.max_steering(max_steering)
			.color(color)
			.window_size(model.window_size)
			.build();

		model
			.boid_list
			.push(boid);
	}

	model
}

fn update(_app: &App, model: &mut Model, _update: Update) { model.update(); }

fn view(_app: &App, model: &Model, frame: Frame)
{
	let draw = _app.draw();

	draw.background()
		.color(Srgb::<f32>::new(0.05, 0.1, 0.15));

	model.display(&draw);

	let line_start = Vec2::new(-200.0, 100.0);
	let line_end = Vec2::new(200.0, -100.0);

	draw.line()
		.start(line_start)
		.end(line_end)
		.caps_round()
		.stroke_weight(1.5)
		.color(Srgb::<f32>::new(0.7, 0.7, 0.7));

	draw.to_frame(_app, &frame)
		.unwrap();
}

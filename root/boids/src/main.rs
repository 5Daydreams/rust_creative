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
		boid_count: 2500,
		boid_list: Vec::new(),
		window_size: [1200, 800],
	};

	_app.new_window()
		.size(model.window_size[0], model.window_size[1])
		.view(view)
		.build()
		.unwrap();

	for _ in 0 .. model.boid_count
	{
		let mut rand_vec: Vec2 = Vec2::new(0., 0.);

		let x_range_pos = -(model.window_size[0] as f32) .. (model.window_size[0] as f32);
		let y_range_pos = -(model.window_size[1] as f32) .. (model.window_size[1] as f32);

		rand_vec.randomize(x_range_pos, y_range_pos);
		let position: Vec2 = rand_vec;

		let x_range_vel = -5.0 .. 7.;
		let y_range_vel = -5.0 .. 6.;

		rand_vec.randomize(x_range_vel, y_range_vel);
		let velocity: Vec2 = rand_vec;

		let body_radius: f32 = rand::thread_rng().gen_range(2.5 .. 3.0);
		let perception_radius: f32 = 150.;
		let max_speed: f32 = 6.5;
		let steering_factor: f32 = 0.012;
		let cohesion_factor: f32 = 0.04;
		let separation_factor: f32 = 0.05;

		let r: f32 = rand::thread_rng().gen_range(0.08 .. 0.15);
		let g: f32 = rand::thread_rng().gen_range(0.02 .. 0.08);
		let b: f32 = rand::thread_rng().gen_range(0.45 .. 0.85);

		let color = Srgb::<f32>::new(r, g, b);

		let boid: Boid = Boid::new()
			.position(position)
			.velocity(velocity)
			.body_radius(body_radius)
			.perception_radius(perception_radius)
			.max_speed(max_speed)
			.steering_factor(steering_factor)
			.cohesion_factor(cohesion_factor)
			.separation_factor(separation_factor)
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

	draw.to_frame(_app, &frame)
		.unwrap();
}

use nannou::prelude::*;

pub trait Nannou
{
	fn update(&mut self);
	fn display(&self, draw: &nannou::Draw);
}


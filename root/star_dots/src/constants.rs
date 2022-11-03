use std::ops::Range;

use nannou::prelude::PI;

pub const POINT_COUNT: u32 = 80;
pub const DEFAULT_WINDOW_SIZE: (u32, u32) = (800, 600);
pub const FOV: f32 = PI * 0.5;
pub const CLIPPING_PLANES: (f32, f32) = (1.0, 300.0);

pub const SPAWN_RANGE_X: Range<f32> = -130.0 .. 130.0;
pub const SPAWN_RANGE_Y: Range<f32> = -80.0 .. 80.0;
pub const SPAWN_RANGE_Z: Range<f32> = -130.0 .. 130.0;

pub const OFFSET_X: f32 = 0.;
pub const OFFSET_Y: f32 = 0.;
pub const OFFSET_Z: f32 = 200.;

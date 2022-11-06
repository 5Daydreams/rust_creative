use nannou::prelude::*;

use crate::{constants::*, trait_nannou::Nannou, vec_extensions::Perspective2D};
use typed_builder::TypedBuilder;

const EDGE_STRENGTH: f32 = 5.;

#[derive(TypedBuilder, Default, Clone)]
pub struct Edge {
    pub start: crate::dot::Dot,
    pub end: crate::dot::Dot,

    #[builder(default = Srgb::<f32>::new(0.05,0.1,0.3))]
    pub color: Srgb<f32>,
}

impl Nannou for Edge {
    fn display(&self, draw: &nannou::Draw) {
        let window_size = [DEFAULT_WINDOW_SIZE.0, DEFAULT_WINDOW_SIZE.1];
        let offset = OFFSET_VEC;

        let display_pos_a = self.start.pos.project_into_2d(
            offset,
            window_size,
            FOV,
            CLIPPING_PLANES.0,
            CLIPPING_PLANES.1,
        );

        let display_pos_b = self.end.pos.project_into_2d(
            offset,
            window_size,
            FOV,
            CLIPPING_PLANES.0,
            CLIPPING_PLANES.1,
        );

        let dist = (self.end.pos).distance(self.start.pos);

        let funny_number = EDGE_STRENGTH / dist;

        let default_line_color =
            Srgb::<f32>::new(0.03, 0.06 + 0.08 * funny_number, 0.15 + 0.1 * funny_number);

        draw.line()
            .caps_round()
            .start(display_pos_a)
            .end(display_pos_b)
            .color(default_line_color);
    }
}

use constants::*;
use dot::Dot;
use edge::Edge;
use nannou::{geom::point, prelude::*};

mod constants;
mod dot;
mod edge;
mod range_cube;
mod trait_nannou;
mod vec_extensions;

use range_cube::*;
use trait_nannou::Nannou;
use vec_extensions::RotateMatrixBaked;

pub struct Model {
    pub curr_time: f32,
    pub prev_time: f32,
    pub delta_time: f32,
    pub point_list: Vec<Dot>,
    pub edge_list: Vec<Edge>,
    pub bounding_box: RangeCube,
    pub window_size: (u32, u32),
}

impl Model {
    pub fn new() -> Self {
        use constants::*;
        let box_ranges: RangeCube = RangeCube::new(SPAWN_RANGE_X, SPAWN_RANGE_Y, SPAWN_RANGE_Z);

        Self {
            curr_time: 0.,
            prev_time: 0.,
            delta_time: 0.16,
            point_list: Vec::new(),
            edge_list: Vec::new(),
            bounding_box: box_ranges,
            window_size: DEFAULT_WINDOW_SIZE,
        }
    }

    pub fn update(&mut self) {
        for point in &mut self.point_list {
            point.pos = point.pos.rotate_y(self.delta_time / 2.);
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Nannou for Model {
    fn display(&self, draw: &nannou::Draw) {
        for point in &self.point_list {
            point.display(draw);
        }

        for edge in &self.edge_list {
            edge.display(draw);
        }
    }
}

pub fn model(_app: &App) -> Model {
    let mut model: Model = Model::new();

    _app.new_window()
        .size(model.window_size.0, model.window_size.1)
        .title("Gradient Dots :v")
        .view(view)
        .build()
        .unwrap();

    for _ in 0..POINT_COUNT {
        let random_pos = model.bounding_box.get_random_vec3();

        let point = Dot::builder().pos(random_pos).build();

        model.point_list.push(point);
    }

    // for &point_a in &model.point_list {
    //     if point_a.neighbours >= 3 {
    //         continue;
    //     }

    //     let list_size = 3 - point_a.neighbours;

    //     let mut closest_list: Vec<(Dot, f32)> = Vec::with_capacity(list_size);

    //     for &point_b in &model.point_list {
    //         if point_a == point_b || point_b.neighbours >= 3 {
    //             continue;
    //         }

    //         let dist = point_a.pos.distance(point_b.pos);

    //         if closest_list.len() < list_size {
    //             closest_list.push((point_b, dist));
    //         } else if dist < closest_list[2].1 {
    //             closest_list[2] = (point_b, dist);
    //         }

    //         closest_list.sort_by(|(_, l), (_, r)| l.total_cmp(r));
    //     }

    //     for mut neighbour in closest_list {
    // 		model.edge_list.push(Edge{ start: &point_a, end: todo!(), color: todo!() });
    //         neighbour.0.neighbours += 1;
    //     }

    //     // point_a.neighbours = 3;

    //     // this should NOT be necessary, but oh well
    //     // assert_eq!(point_a.neighbours, 3);
    // }

    for i in 0..model.point_list.len() {
        let current = &model.point_list[i];
        let neighbor_count = 3 - current.neighbours;
        if neighbor_count == 0 {
            continue;
        }
        let mut dists = model
            .point_list
            .iter()
            .enumerate()
            .filter(|&(index_other, other)| other.neighbours < neighbor_count && index_other != i)
            .map(|(index_other, other)| (current.pos.distance(other.pos), index_other))
            .collect::<Vec<_>>();

        dists.sort_by(|(l, _), (r, _)| l.total_cmp(r));
        dists.truncate(neighbor_count);

        for (_, other_index) in dists {
            let thing = Edge {
                start: model.point_list[other_index],
                end: model.point_list[i],
                color: Srgb::<f32>::new(0.05, 0.3, 0.4),
            };
            model.edge_list.push(thing);
        }
    }

    model
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.curr_time = _app.time;
    model.delta_time = model.curr_time - model.prev_time;

    model.update();
    model.prev_time = model.curr_time;
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let bg_color: Srgb<f32> = Srgb::<f32>::new(0.01, 0.05, 0.1);

    draw.background().color(bg_color);

    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}

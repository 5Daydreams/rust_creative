use nannou::prelude::*;

mod constants;
// use image::{Rgb, RgbImage};
#[allow(unused_imports)]
use rand::{self, Rng};

pub struct Model {
    pub curr_time: f32,
    pub prev_time: f32,
    pub delta_time: f32,
    pub window_size: (u32, u32),
}

impl Model {
    pub fn new() -> Self {
        use constants::*;

        Self {
            curr_time: 0.,
            prev_time: 0.,
            delta_time: 0.16,
            window_size: DEFAULT_WINDOW_SIZE,
        }
    }

    pub fn update(&mut self) {
        // TODO: add update behavior to within the model struct, if necessary
    }

    pub fn draw_eye(&self, draw: &nannou::Draw, eye_pos: Vec2, eye_offset: f32, r1: f32, r2: f32) {
        draw.ellipse()
            .x(eye_pos.x + eye_offset)
            .y(eye_pos.y)
            .color(WHITE)
            .radius(r1);

        draw.ellipse()
            .x(eye_pos.x + eye_offset)
            .y(eye_pos.y)
            .color(BLACK)
            .radius(r2);

        draw.ellipse()
            .x(eye_pos.x + eye_offset - r1 / 2.0)
            .y(eye_pos.y + r1 / 2.0)
            .color(WHITE)
            .radius(r1 / 5.0);
    }

    pub fn display(&self, draw: &nannou::Draw) {
        // TODO: add behavior to display things on update

        let x = 10.0 * self.curr_time.sin();
        let y = 70.0;
        let center_pos = vec2(x, y);
        let eye_offset = 40.0;
        let r1 = 25.0;
        let r2 = 20.0;

        self.draw_eye(draw, center_pos, eye_offset, r1, r2);
        self.draw_eye(draw, center_pos, -eye_offset, r1, r2);
    }
}

pub fn model(_app: &App) -> Model {
    #[allow(unused_mut)]
    let mut model: Model = Model::new();

    _app.new_window()
        .size(model.window_size.0, model.window_size.1)
        .title("Texture Galaxy")
        .view(view)
        .build()
        .unwrap();

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

    let assets = app.assets_path().unwrap();
    let img_path = assets.join("textures").join("nannou.png");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    draw.texture(&texture);
    
    use nannou::image::{RgbImage, Rgb};
    
    let mut img = RgbImage::new(32, 32);
    
    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }
    
    let texture = wgpu::Texture::from_image(app, img);
    draw.texture(&texture);
    
    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}

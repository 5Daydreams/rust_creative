// nannou provides this thing called a prelude to help with handling reusage of code from other scripts;
use nannou::prelude::{rgb::Rgb, *};
// Interestingly enough, rgb::Rgb was not added by me! VSCode does that Rider-thing where it checks for usage and includes the libs/files

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw: Draw = _app.draw();

    let bg_color: Rgb = Rgb::new(0.25, 0.05, 0.45);
    // Clear the background to purple.
    draw.background().color(bg_color);

    let sine: f32 = _app.time.sin();

    let slower_sine: f32 = (_app.time / 2.0).sin();

    let boundary: Rect = _app.window_rect();

    let x: f32 = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
    let y: f32 = map_range(slower_sine, -1.0, 1.0, boundary.bottom(), boundary.top());

    // Draw a blue ellipse with a radius of 10 at the (x,y) coordinates of (0.0, 0.0)
    draw.ellipse().color(STEELBLUE).x_y(x, y);

    draw.to_frame(_app, &frame).unwrap();
}

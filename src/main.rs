// nannou provides this thing called a prelude to help with handling reusage of code from other scripts;
use nannou::prelude::{rgb::Rgb, *};

fn main() {
    // sketches take no input/output, apps are more complex
    // nannou::sketch(view).run();
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(_app: &App, _model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = _app.draw();

    let bgColor: Rgb = Rgb::new(0.15, 0.05, 0.4);
    draw.background().color(bgColor);

    let iterationCount: i32 = 500;

    let points = (0..iterationCount).map(|i| {
        let x = (i - iterationCount / 2) as f32 / 10.0; //subtract 25 to center the sine wave
        let y: f32 = x.sin();
        let point = pt2(x, y) * 20.0; //scale sine wave by 20.0

        let sineValue = (y + 1.0) * 0.5;

        let sineColor: Rgb = Rgb::new(0.01, 0.2, 0.6 * sineValue);
        (point, sineColor)
    });
    draw.polyline().weight(3.0).points_colored(points);

    // put everything on the frame
    draw.to_frame(_app, &frame).unwrap();
}

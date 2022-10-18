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

    let bgColor: Rgb = Rgb::new(0.3, 0.05, 0.6);
    draw.background().color(bgColor);

    let ellipseColor: Rgb = Rgb::new(0.01, 0.4, 0.6);
    draw.ellipse()
        .x_y(150.0, -80.0)
        .w_h(150.0, 280.0)
        .color(ellipseColor);

    let points = (0..50).map(|i| {
        let x = i as f32 - 25.0; //subtract 25 to center the sine wave
        let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);

    // put everything on the frame
    draw.to_frame(_app, &frame).unwrap();
}

use nannou::{App, LoopMode, window, Frame, color::*, prelude::Update};

use day24::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    _input: String,
}

impl Model {
    fn new(window: window::Id) -> Self {
        let input = input_txt(InputFile::Real);
        Model {
            _window: window,
            _input: input,
        }
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    app.set_loop_mode(LoopMode::rate_fps(60.0));
    Model::new(_window)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //println!("since_last {}", update.since_last.as_millis());
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    draw.background().color(PLUM);

    let col_count = 100;
    let row_count = 100;
    let window = app.window_rect();
    let pix_per_col = window.w() / col_count as f32;
    let pix_per_row = window.h() / row_count as f32;

    let gdraw = draw
        // scale from 1 pixel per unit (default) to SIZE pixels per unit
        //.scale(scale / 2.0)
        .scale_x(pix_per_col)
        .scale_y(pix_per_row)
        // flip y axis direction
        .scale_y(-1.0)
        // move the origin to top left of window
        .x_y(
            col_count as f32 / -2.0 + 0.5,
            row_count as f32 / -2.0 + 0.5,
        );

    for y in 0..row_count {
        for x in 0..col_count {
        }
    }

    draw.to_frame(app, &frame).unwrap();
}


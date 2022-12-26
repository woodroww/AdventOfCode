use std::collections::{HashMap, HashSet};

use day23::{parse_input, input_txt, InputFile, ElfMove, Position, any_adjacent_elves, propose_move, max_xy_min_xy};
use nannou::{prelude::Update, App, LoopMode, window, Frame, color::*};

//use crate::input_txt;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    _input: String,
    elves: HashSet<Position>,
    round: usize,
    timer: std::time::Duration,
    /*x_size: isize,
    y_size: isize,
    x_offset: isize,
    y_offset: isize,*/
}

impl Model {
    fn new(window: window::Id) -> Self {
        let input = input_txt(InputFile::Real);
        let elves = parse_input(&input);

        Model {
            _window: window,
            _input: input,
            elves,
            round: 0,
            timer: std::time::Duration::from_secs(0),
            /*x_size,
            y_size,
            x_offset,
            y_offset,*/
        }
    }
}

fn advent(model: &mut Model) {
    if model.round >= 1029 {
        return;
    }
    let mut moves: Vec<ElfMove> = Vec::new();
    let mut to_spots: HashMap<Position, usize> = HashMap::new();
    for elf in model.elves.iter() {
        if any_adjacent_elves(elf, &model.elves) {
            let proposed = propose_move(elf, &model.elves, model.round);
            if let Some(proposed_move) = proposed {
                let e = to_spots.entry(proposed_move.to.clone()).or_insert(0);
                *e += 1;
                moves.push(proposed_move);
            }
        }
    }
    let mut good_moves: Vec<ElfMove> = Vec::new();
    for m in moves.into_iter() {
        //println!("proposed move: {}", m);
        if *to_spots.get(&m.to).unwrap() == 1 {
            good_moves.push(m);
        }
    }

    for m in good_moves.iter() {
        model.elves.remove(&m.from);
    }
    for m in good_moves.into_iter() {
        model.elves.insert(m.to);
    }
    model.round += 1;
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(800, 800).view(view).build().unwrap();
    app.set_loop_mode(LoopMode::rate_fps(60.0));
    Model::new(_window)
}

fn update(_app: &App, model: &mut Model, update: Update) {
    //println!("since_last {}", update.since_last.as_millis());
    let now = update.since_start.as_millis();
    if now - model.timer.as_millis() > 100 {
        model.timer = update.since_start;
        advent(model);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let (max_x, max_y, min_x, min_y) = max_xy_min_xy(&model.elves);
    //println!("max:({},{}) min:({},{})", max_x, max_y, min_x, min_y);
    let x_size = max_x - min_x + 1;
    let y_size = max_y - min_y + 1;
    let x_offset = min_x;
    let y_offset = min_y;
    // 12x11 part_1 example
    // 83x82 part_1 real

    let col_count = x_size;
    let row_count = y_size;
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

    let elf_color = Srgb::new(250u8, 102, 206);

    for y in 0..row_count {
        for x in 0..col_count {
            if model.elves.contains(&Position { x: x as isize + x_offset, y: y as isize + y_offset }) {
                let cdraw = gdraw.x_y(x as f32, y as f32);
                cdraw.rect().color(elf_color).w_h(1.0, 1.0);
            } else {
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}


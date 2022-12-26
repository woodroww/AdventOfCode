use nannou::prelude::*;

use day14::*;

fn main() {
    nannou::app(model).update(update).run();
}


struct Model {
    _window: window::Id,
    _input: String,
    cave: Cave,
    last_update: f32,
    sand_moved: bool,
    off_map: bool,
    spawner_blocked: bool,
    landed: usize,
}

impl Model {
    fn new(window: window::Id) -> Self {
        let input = input_txt(InputFile::Real);
        let cave = Cave::new(&input, true);
        Model {
            _window: window,
            _input: input,
            cave,
            last_update: 0.0,
            sand_moved: true,
            off_map: false,
            landed: 0,
            spawner_blocked: false,
        }
    }
}

fn model(app: &App) -> Model {
    // window: w:697, h:1382)
    let _window = app.new_window().size(697, 1382).view(view).build().unwrap();
    app.set_loop_mode(LoopMode::rate_fps(60.0));
    Model::new(_window)
}

pub fn sand_step(cave: &mut Cave) -> MoveType {
    if cave.moving_sand.is_none() {
        match cave.can_move_sand_at(&cave.spawner) {
            MoveType::MoveInto(new_move) => {
                cave.moving_sand = Some(new_move);
            }
            the_move => { return the_move }
        }
    }
    cave.step()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //println!("since_last {}", update.since_last.as_millis());
    let how_many = 500;
    let mut count = 0;
    while model.off_map == false && model.spawner_blocked == false && count < how_many {
        match sand_step(&mut model.cave) {
            MoveType::OffMap => {
                model.off_map = true;
                //println!("landed: {}", model.landed);
            }
            MoveType::Blocked => {
                model.spawner_blocked = true;
                println!("Blocked landed: {}", model.landed);
            }
            MoveType::Resting => {
                model.landed += 1;
                //println!("landed: {}", model.landed);
            }
            MoveType::MoveInto(_) => {
                model.sand_moved = true;
            }
        }
        count += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();
    draw.background().color(PLUM);

    let window = app.window_rect();
    let col_count = model.cave.x_size + 1;
    let row_count = model.cave.y_size + 1;
    
    let pix_per_col =  window.w() / col_count as f32;
    let pix_per_row = window.h() / row_count as f32;
    /*
    println!("col_count {}", col_count);
    println!("pix_per_col {}", pix_per_col);
    println!("row_count {}", row_count);
    println!("pix_per_row {}", pix_per_row);
    */

    let scale = 1.0;

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

    let falling_sand = model.cave.moving_sand;

    for y in 0..row_count {
        for x in 0..col_count {
            // center coordinates at the current square
            let cdraw = gdraw.x_y(x as f32, y as f32);
            if falling_sand.is_some() && falling_sand.unwrap() == (Position { x, y }) {
                cdraw.rect().color(SANDYBROWN).w_h(1.0, 1.0);
            } else {
                match model.cave.things[y][x] {
                    CaveItem::Sand => {
                        cdraw.rect().color(GOLDENROD).w_h(1.0, 1.0);
                    }
                    CaveItem::Rock => {
                        cdraw.rect().color(SADDLEBROWN).w_h(1.0, 1.0);
                    }
                    CaveItem::Air => {
                        cdraw.rect().color(DARKBLUE).w_h(1.0, 1.0);
                    }
                }
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}


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
    landed: usize,
}

impl Model {
    fn new(window: window::Id) -> Self {
        let input = input_txt(InputFile::Real);
        let cave = Cave::new(&input);
        Model {
            _window: window,
            _input: input,
            cave,
            last_update: 0.0,
            sand_moved: true,
            off_map: false,
            landed: 0,
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
            _ => {}
        }
    }
    cave.step()
}

fn update(app: &App, model: &mut Model, update: Update) {
    //println!("since_last {}", update.since_last.as_millis());
    let how_many = 15;
    let mut count = 0;
    while model.off_map == false && count < how_many {
        match sand_step(&mut model.cave) {
            MoveType::OffMap => {
                model.off_map = true;
                println!("landed: {}", model.landed);
            }
            MoveType::Blocked => {
                unreachable!();
            }
            MoveType::Resting => {
                model.landed += 1;
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
    let cols_count = model.cave.x_size + 1;
    let rows_count = model.cave.y_size + 1;
    
    //let aspect_ratio = cols_count / rows_count;
    //let window_aspect = window.w() / window.h();

    let scale = if window.h() > window.w() {
        window.w() / cols_count as f32
    } else {
        window.h() / rows_count as f32
    };

    let gdraw = draw
        // scale from 1 pixel per unit (default) to SIZE pixels per unit
        .scale(scale)
        // flip y axis direction
        .scale_y(-1.0)
        // move the origin to top left of window
        .x_y(
            cols_count as f32 / -2.0 + 0.5,
            rows_count as f32 / -2.0 + 0.5,
        );

    let falling_sand = model.cave.moving_sand;

    for y in 0..rows_count {
        for x in 0..cols_count {
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

/*fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(&input)); // 530 too low
    //println!("Part 2: {}", part_2(&input));
}*/

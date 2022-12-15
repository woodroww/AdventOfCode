use std::collections::HashSet;

use nannou::{
    color::*, event::Update, prelude::Point2, state::mouse::Button, window, App, Frame, LoopMode,
};

use day15::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    _input: String,
    sensors: Vec<Sensor>,
    cave_rect: Rectangle,
    draw_grid: Vec<Vec<Srgb<u8>>>,
    saved_grid: Vec<Vec<Srgb<u8>>>,
    mouse_cell: Option<Position<isize>>,
}

impl Model {
    fn new(window: window::Id) -> Self {
        let input = input_txt(InputFile::Example);
        let sensors = parse_input(&input);
        let (max_x, max_y, min_x, min_y) = max_xy_min_xy(&sensors);
        println!("min_x:{} max_x:{}", min_x, max_x);
        println!("min_y:{} max_y:{}", min_y, max_y);
        let cave_rect = Rectangle {
            top_left: Position { x: min_x, y: min_y },
            bottom_right: Position {
                x: max_x + 1,
                y: max_y + 1,
            },
        };
        println!("begin rect: {}", cave_rect);
        println!("begin rect width: {}", cave_rect.width());
        println!("begin rect height: {}", cave_rect.height());
        let draw_grid = set_up_draw_grid(&cave_rect, &sensors);
        Model {
            _window: window,
            _input: input,
            sensors,
            cave_rect,
            draw_grid: draw_grid.clone(),
            saved_grid: draw_grid,
            mouse_cell: None,
        }
    }
}

fn set_up_draw_grid(cave_rect: &Rectangle, sensors: &Vec<Sensor>) -> Vec<Vec<Srgb<u8>>> {
    let mut rows = Vec::new();

    let beacon_color = Srgb::new(99u8, 6, 71);
    let sensor_color = Srgb::new(252u8, 114, 86);
    let backgroud_color = Srgb::new(66u8, 135, 245);

    let col_count = cave_rect.width();
    let row_count = cave_rect.height();

    for y in 0..=row_count {
        let mut cols = Vec::new();
        let sensors_for_row = sensors
            .iter()
            .filter(|s| s.location.y == y as isize + cave_rect.y_offset());
        let beacons_for_row = sensors
            .iter()
            .filter(|s| s.beacon.y == y as isize + cave_rect.y_offset());
        for x in 0..=col_count {
            let mut sensor = sensors_for_row
                .clone()
                .filter(|s| s.location.x == x as isize + cave_rect.x_offset());
            let mut beacon = beacons_for_row
                .clone()
                .filter(|s| s.beacon.x == x as isize + cave_rect.x_offset());
            if let Some(_s) = sensor.next() {
                cols.push(sensor_color);
            } else if let Some(_b) = beacon.next() {
                cols.push(beacon_color);
            } else {
                cols.push(backgroud_color);
            }
        }
        rows.push(cols);
    }
    rows
}

fn mouse_point_to_cell(app: &App, model: &mut Model, point: Point2) -> Position<isize> {
    let w = model.cave_rect.width();
    let h = model.cave_rect.height();
    let window = app.window_rect();
    let window_w = window.w();
    let window_h = window.h();
    let mouse_x = window_w / 2.0 + point.x;
    let mouse_y = window_h - (window_h / 2.0 + point.y);
    let pix_per_col = window_w / w as f32;
    let pix_per_row = window_h / h as f32;
    let x_cell = (mouse_x / pix_per_col).floor() as isize + model.cave_rect.x_offset();
    let y_cell = (mouse_y / pix_per_row).floor() as isize + model.cave_rect.y_offset();
    Position {
        x: x_cell,
        y: y_cell,
    }
}

fn mouse_moved(app: &App, model: &mut Model, point: Point2) {
    let cell = mouse_point_to_cell(app, model, point);
    model.mouse_cell = Some(cell);
}

fn my_solution(model: &mut Model) {
    if let Some(cell) = model.mouse_cell {
        //println!("{:?} click in cell {}", button, cell);
        let sensor = model.sensors.iter().find(|s| s.location.x == cell.x && s.location.y == cell.y);

        if let Some(sensor) = sensor {
            let mut sensor_reach = HashSet::new();
            for y in model.cave_rect.y_min()..model.cave_rect.y_max() {
                let min_x = model.cave_rect.x_min();
                let max_x = model.cave_rect.x_max();
                for x in min_x..=max_x {
                    if sensor.covers_point(&Position { x, y }) {
                        sensor_reach.insert(Position { x, y });
                    }
                }
            }
            for covered in &sensor_reach {
                let y_idx = covered.y - model.cave_rect.y_offset();
                let x_idx = covered.x - model.cave_rect.x_offset();
                if y_idx < model.draw_grid.len() as isize {
                    //model.draw_grid[y_idx as usize][x_idx as usize] = Srgb::new(50u8, 50, 50);
                    model.draw_grid[y_idx as usize][x_idx as usize] = Srgb::new(1u8, 12, 74);
                }
            }
        } else {
            model.draw_grid = model.saved_grid.clone();
        }
    }
}

fn mouse_clicked(_app: &App, model: &mut Model, button: Button) {
    my_solution(model);
    /*
    if let Some(cell) = model.mouse_cell {
        //println!("{:?} click in cell {}", button, cell);
        let sensor = model.sensors.iter().find(|s| s.location.x == cell.x && s.location.y == cell.y);
        if let Some(sensor) = sensor {
            //println!("sensor.location {}", sensor.location);
            //println!("sensor.md {}", sensor.manhattan_distance());
            let sensor_reach = set_for_sensor(&sensor, &model.cave_rect);
            for covered in sensor_reach {
                //println!("covered {}", covered);
                let y_idx = covered.y - model.cave_rect.y_offset();
                let x_idx = covered.x - model.cave_rect.x_offset();
                if y_idx < model.draw_grid.len() as isize {
                    model.draw_grid[y_idx as usize][x_idx as usize] = Srgb::new(50u8, 50, 50);
                }
            }
        } else {
            model.draw_grid = model.saved_grid.clone();
        }
        println!();
    }
    */
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(800, 800)
        .view(view)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_clicked)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::rate_fps(60.0));
    Model::new(_window)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //println!("since_last {}", update.since_last.as_millis());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);

    let col_count = model.cave_rect.width();
    let row_count = model.cave_rect.height();
    let window = app.window_rect();
    let pix_per_col = window.w() / col_count as f32;
    let pix_per_row = window.h() / row_count as f32;

    let mouse_beacon_color = Srgb::new(224u8, 16, 162);
    let beacon_highlight = Srgb::new(250u8, 102, 206);
    let mouse_sensor_color = Srgb::new(143u8, 63, 47);
    let sensor_highlight = Srgb::new(227u8, 136, 118);

    //println!("{}", model.rect);
    //println!("row_count: {}", row_count);
    //println!("col_count: {}", col_count);

    let gdraw = draw
        // scale from 1 pixel per unit (default) to SIZE pixels per unit
        //.scale(scale / 2.0)
        .scale_x(pix_per_col)
        .scale_y(pix_per_row)
        // flip y axis direction
        .scale_y(-1.0)
        // move the origin to top left of window
        .x_y(col_count as f32 / -2.0 + 0.5, row_count as f32 / -2.0 + 0.5);

    for y in 0..=row_count {
        for x in 0..=col_count {
            let cdraw = gdraw.x_y(x as f32, y as f32);
            cdraw.rect().color(model.draw_grid[y][x]).w_h(1.0, 1.0);
        }
    }

    for s in model.sensors.iter() {
        let x = s.location.x - model.cave_rect.x_offset();
        let y = s.location.y + model.cave_rect.y_offset();
        let cdraw = gdraw.x_y(x as f32, y as f32);
        cdraw
            .rect()
            .no_fill()
            .stroke(sensor_highlight)
            .w_h(1.0, 1.0)
            .stroke_weight(0.05);

        let x = s.beacon.x - model.cave_rect.x_offset();
        let y = s.beacon.y + model.cave_rect.y_offset();
        let cdraw = gdraw.x_y(x as f32, y as f32);
        cdraw
            .rect()
            .no_fill()
            .stroke(beacon_highlight)
            .w_h(1.0, 1.0)
            .stroke_weight(0.05);
    }


    if let Some(cell) = model.mouse_cell {
        let adj_x = cell.x - model.cave_rect.x_offset();
        let adj_y = cell.y - model.cave_rect.y_offset();
        //let org_color = model.draw_grid[adj_y as usize][adj_x as usize];
        let cdraw = gdraw.x_y(adj_x as f32, adj_y as f32);
        cdraw
            .rect()
            .no_fill()
            .stroke(BLACK)
            .w_h(1.0, 1.0)
            .stroke_weight(0.05);
    }
    draw.to_frame(app, &frame).unwrap();
}

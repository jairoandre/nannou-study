use nannou::prelude::*;

const N: u32 = 50;
const H_N: u32 = N / 2;
const NN: u32 = N*N;
const SCL: f32 = 10.0;

const H_SCL: f32 = SCL * 0.5;
const SIZE: f32 = N as f32 * SCL;
const H_SIZE: f32 = SIZE * 0.5;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn real_coords(x: u32, y: u32) -> Vector2<f32> {
    let r_x = x as f32 * SCL - H_SIZE + H_SCL;
    let r_y = y as f32 * SCL - H_SIZE + H_SCL;
    vec2(r_x, r_y)
}

fn x_y_to_index(x: u32, y: u32) -> usize {
    (x + y * N) as usize
}

enum Direction {
    U, D, L, R
}

struct Model {
    grid: [f32; NN as usize],
    head: Direction,
    position: Vector2<u32>
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(SIZE as u32, SIZE as u32)
        .view(view)
        .build()
        .unwrap();
    let grid = [1.0; NN as usize];
    Model {
        grid,
        head: Direction::R,
        position: vec2(H_N, H_N)
    }
}

fn resolve(x: u32, y: u32, state: f32, from: & Direction) -> (u32, u32, Direction, f32) {
    let to = match from {
        Direction::U   => if state == 1.0 && x < N - 1  { Direction::R } else { Direction::L },
        Direction::R   => if state == 1.0 && y < N - 1 { Direction::D } else { Direction::U },
        Direction::D   => if state == 1.0 && x > 0 { Direction::L } else { Direction::R },
        _   => if state == 1.0 && y > 0 { Direction::U } else { Direction::D }
    };
    
    let (nx, ny): (u32, u32) = match to {
        Direction::U   => (x, y + 1),
        Direction::R   => (x + 1, y),
        Direction::D   => (x - 1, y),
        _ => (x, y - 1)
    };
    (nx, ny, to, if state == 1.0 { 0.0 } else { 1.0 })
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let p = model.position;
    let idx = x_y_to_index(p.x, p.y);
    let state = model.grid[idx];
    let (n_x, n_y, n_head, n_state) = resolve(p.x, p.y, state, & model.head);
    model.grid[idx] = n_state;
    model.position = vec2(n_x, n_y);
    model.head = n_head;
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(WHITE);
    }
    let grid = model.grid;
    for x in 0..N as u32 {
        for y in 0..N as u32 {
            let state = grid[x_y_to_index(x, y)];
            draw.rect().xy(real_coords(x, y)).w_h(SCL, SCL).color(srgb(state, state, state));
        }

    }
    draw.to_frame(app, &frame).unwrap();
}
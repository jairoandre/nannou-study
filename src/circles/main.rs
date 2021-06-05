use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

const N: usize = 20000;

struct State {
    x: f32,
    r: f32,
    iteration: u32,
}

impl State {
    fn init() -> Self {
        return Self {
            x: random_f32(),
            r: random_f32() * 4.0 + 0.5,
            iteration: 0,
        }
    }
}

struct Model {
    _window: window::Id,
    states: Vec<State>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    Model {
        _window,
        states: (0..N).map(|_| State::init()).collect()
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let prob = 1. - sigmoid((app.elapsed_frames() as f32 + 50.) as f32 / 500.);
    for state in model.states.iter_mut() {
        if random_f32() < prob {
            *state = State::init();
        } else {
            state.x = state.r * state.x * (1.0 - state.x);
            state.iteration += 1;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    const SCALE_X: f32 = 1024.0 * 1.4;
    const SCALE_Y: f32 = 1024.0 * 0.4;

    if app.elapsed_frames() < 2 {
        draw.background().color(PLUM);
    } else {
        // draw.background().rgba(1.0, 1.0, 1.0, 0.001).;
        // draw.alpha_blend(BLEND_NORMAL).background().rgba(1.0, 1.0, 1.0, 0.001);
        // draw.rect()
        //     .rgba(1.0, 1.0, 1.0, 0.002)
        //     .x(0.2 * SCALE_X)
        //     .y(-0.15 * SCALE_Y)
        //     .width(0.15 * SCALE_X)
        //     .height(0.4 * SCALE_Y)
        //     ;
    }

    let colorfac = sigmoid((500. - app.elapsed_frames() as f32) as f32 / 1500.);

    for state in model.states.iter() {
        let x = state.r / 5.0;
        let y = state.x;
        let it_fac = 1.0 / (state.iteration as f32 + 5.) + 1./5.;
        draw
            // .blend(BLEND_DARKEST)
            // .point_mode()
            .ellipse()
            .rgba(colorfac * 0.9, colorfac * 0.3, colorfac * 0.2, (1.0 - it_fac) * 0.05)
            .x(x * SCALE_X - SCALE_X/2.0)
            .y(y * SCALE_Y - SCALE_Y/2.0)
            .radius(0.05 + it_fac * 4.0)
            ;
    }
    draw.to_frame(app, &frame).unwrap();
}

fn sigmoid(x: f32) -> f32 {
    return 1.0 / (1.0 + (-x).exp());
}
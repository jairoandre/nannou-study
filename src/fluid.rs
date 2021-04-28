use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

const N: f32 = 250.0;

struct Fluid {
    size: f32,
    dt: f32,
    diff: f32,
    visc: f32,
    s: Vec<f32>,
    density: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    vx0: Vec<f32>,
    vy0: Vec<f32>,
}
impl Fluid {
    pub fn new(dt: f32, diff: f32, visc: f32) -> Self {
        Fluid {
            size: N,
            dt,
            diff,
            visc,
            s: Vec::new(),
            density: Vec::new(),
            vx: Vec::new(),
            vy: Vec::new(),
            vx0: Vec::new(),
            vy0: Vec::new(),
        }

    }
}

struct Model {
    fluid: Fluid
}

fn model(_app: &App) -> Model {
    Model {
        fluid: Fluid::new(0.1, 0.1, 0.1)
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let fluid = &mut model.fluid;
    fluid.s.push(1.0);

}

fn view(_app: &App, _model: &Model, frame: Frame){
    frame.clear(PURPLE);
}
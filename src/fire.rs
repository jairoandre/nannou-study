use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

const SIZE: f32 = 500.0;
const HALF_SIZE: f32 = SIZE * 0.5;
const SCL: f32 = 10.0;
const HALF_SCL: f32 = SCL * 0.5;
const SIZE_SCL: f32 = SIZE / SCL;
const N: f32 = SIZE_SCL * SIZE_SCL;

fn idx_to_x_y(idx: usize) -> (f32, f32) {
    let s = SIZE_SCL as usize;
    let x = (idx % s) as f32 * SCL - HALF_SIZE + HALF_SCL;
    let y = (idx / s) as f32 * SCL - HALF_SIZE + HALF_SCL;
    (x, y)
}

struct Model {
    particles: Vec<Particle>,
    frequency: f32,
    r_phase: f32,
    g_phase: f32,
    b_phase: f32
}

struct Particle {
    intensity: u32,
}
impl Particle {
    pub fn new(intensity: u32) -> Self {
        Particle {
            intensity
        }
    }
}

fn model(_app: &App) -> Model {
    let mut particles: Vec<Particle> = Vec::new();
    for i in 0..N as u32 {
        let _intensity = if i > (N - SIZE_SCL) as u32 { 35 } else { 0 };
        particles.push(Particle::new(i as u32))
    }
    Model {
        particles
    }
}

fn intensity_to_color(intensity: u32) -> Rgb {
    let i = intensity as f32;
    let r = (i * 0.3).sin() * 0.5 + 0.5;
    let g = (i * 0.3 + 4.0).sin() * 0.5 + 0.3;
    let b = (i * 0.3 + 6.0).sin() * 0.1 + 0.05;
    srgb(r, g, b)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);
    for (idx, particle) in model.particles.iter().enumerate() {
        let (x, y) = idx_to_x_y(idx);
        draw.rect().x_y(x, y).w_h(SCL, SCL).color(intensity_to_color(particle.intensity));
    }
    draw.to_frame(app, &frame).unwrap()
}
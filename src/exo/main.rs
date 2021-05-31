use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Particle {
    pos: Point2,
    vel: Vector2,
}
impl Particle {
    pub fn new(pos: Point2) -> Self {
        Particle {
            pos,
            vel: Vector2::zero(),
        }
    }
    fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.pos).wh(Vector2::one()).color(WHITE);
    }
    fn update(&mut self) {
        self.pos += self.vel;
    }
}

const SPACE: f32 = 500.0;

fn random_pt() -> Point2 {
    let x = (random::<f32>() - 0.5) * SPACE;
    let y = (random::<f32>() - 0.5) * SPACE;
    pt2(x, y)
}

struct Model {
    particles: Vec<Particle>
}

const SIZE: usize = 50;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(800, 600)
        .view(view)
        .build()
        .unwrap();
    let mut particles = Vec::new();
    for _i in 0..SIZE {
        particles.push(Particle::new(random_pt()));
    }
    Model {
        particles
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    if app.elapsed_frames() < 5 {
        draw.background().color(BLACK);
    }
    let window = app.window(frame.window_id()).unwrap();
    let (w, h) = window.inner_size_pixels();
    draw.rect().xy(Point2::zero()).wh(vec2(w as f32, h as f32)).color(srgba(0.0, 0.0, 0.0, 0.01));
    for particle in model.particles.iter() {
        particle.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    particles: Vec<Particle>
}

#[derive(Debug)]
struct Particle {
    position: Vector2<f32>,
    intensity: u8,
}
impl Particle {
    pub fn new(position: Vector2<f32>) -> Self {
        Particle {
            position,
            intensity: 35,
        }
    }

    fn update(&mut self, time: f32) {
        let rand = (random::<f32>() - 1.0) * 3.0;
        self.position.x -= (rand as u8 & 1) as f32;
        self.position.y -= rand;
        self.intensity -= rand as u8 * (time as u8 % 2);
    }

    fn color(& self) -> Rgb {
        let ratio = self.intensity as f32/ 35.0;
        srgb(1.0 * ratio, 0.5 * ratio, 0.1 * ratio)
    }
}

fn rand_position(range: f32) -> Vector2<f32> {
    let x = (random::<f32>() - 0.5) * range;
    let y = (random::<f32>() - 0.5) * range;
    vec2(x, y)
}

fn model(_app: &App) -> Model {
    let mut particles: Vec<Particle> = Vec::new();
    for _i in 0..300 {
        particles.push(Particle::new(vec2(_i as f32 * 1.0, 0.0)))
    }
    Model {
        particles
    }
}

fn remove_dead(particles: &mut Vec<Particle>) {
    particles.retain(|particle| particle.intensity > 0)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let _time = app.elapsed_frames() as f32 / 60.0;
    for particle in model.particles.iter_mut() {
        particle.update(_time);
    }
    remove_dead(&mut model.particles);
    //println!("${:?}", model.particles);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    let window = app.window(frame.window_id()).unwrap();
    let (w, h) = window.inner_size_pixels();
    draw.rect().w_h(w as f32, h as f32).color(srgba(0.0,0.0,0.0,0.2));
    for particle in model.particles.iter() {
        draw.rect()
            .xy(particle.position)
            .w(1.0)
            .h(1.0)
            .color(particle.color());
    }
    draw.to_frame(app, &frame).unwrap();
}

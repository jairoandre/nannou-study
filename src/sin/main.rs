use nannou::prelude::*;
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const W: f32 = 600.0;
const H_W: f32 = W * 0.5;

struct Wave {
  amplitude: f32,
  period: f32,
  phase: f32,
  radius: f32,
}
impl Wave {
  pub fn new(amplitude: f32, period: f32, phase: f32, radius: f32) -> Self {
    Wave {
      amplitude,
      period,
      phase,
      radius,
    }
  }
  fn draw(&self, draw: &Draw) {
    let step = (W / self.radius) as i32;
    for i in 0..step {
      let x = -H_W + self.radius * i as f32;
      let y = (self.phase + TAU * x / self.period).sin() * self.amplitude;
      draw.ellipse().xy(pt2(x, y)).wh(Vector2::one() * self.radius).color(WHITE);
    }
  }
}

struct Model {
  waves: Vec<Wave>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(600, 200)
        .view(view)
        .build()
        .unwrap();
    let mut waves = vec!();
    waves.push(Wave::new(100.0, H_W, 0.0, 10.0));
    Model {
      waves
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

  for wave in model.waves.iter_mut() {
    wave.phase += 0.01;
  }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(PURPLE);
    for wave in model.waves.iter() {
      wave.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
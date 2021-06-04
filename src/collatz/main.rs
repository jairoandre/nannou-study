use nannou::prelude::*;
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Thing {
  start: Point2,
  end: Point2,
  n: u64,
  angle: f32,
  steps: u64,
  color: Srgba,
}
impl Thing {
  fn update(&mut self) {
    if self.n == 1 {
      return;
    }
    let n = collatz(self.n);
    self.angle += if n % 2 == 0 { ANGLE_STEP } else { - ANGLE_STEP };
    let inc = pt2(self.angle.cos(), self.angle.sin()) * SIZE;
    self.start = self.end.clone();
    self.end += inc;
    self.n = n;
    self.steps += 1;
  }
  fn draw(&self, draw: &Draw) {
    if self.n == 1 { return; }
    let offset = pt2(H_W, 0.0);
    draw.polyline().weight(3.0).points(vec!(self.start - offset, self.end - offset)).color(self.color);
  }
}

struct Model {
  things: Vec<Thing>
}

fn collatz(n: u64) -> u64 {
  return if n % 2 == 0 {
    n / 2
  } else {
    (3 * n + 1) / 2
  }
}

const SIZE: f32 = 2.0;
const ANGLE_STEP: f32 = 0.04;//PI / 16.0;
const N: usize = 300;
const W: f32 = 1200.0;
const H: f32 = 1200.0;
const H_W: f32 = W * 0.5;
const H_H: f32 = H * 0.5;
const BASE_COLOR: (f32, f32, f32) = (223.32, 550.12, 0.0);

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(W as u32, H as u32)
        .view(view)
        .build()
        .unwrap();
    let mut things = vec!();
    for _i in 0..N {
      let n = random::<f32>();
      things.push(Thing {
        start: Point2::zero(),
        end: Point2::zero(),
        n: random::<u64>() as u64,
        angle: 0.0,
        steps: 0,
        color: srgba((n*BASE_COLOR.0).sin() * 0.5 + 0.5, (n*BASE_COLOR.1).sin() * 0.5 + 0.5, (n*BASE_COLOR.2).sin() * 0.2 + 0.8, 0.3)
      });
    }
    Model {
      things
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
  for thing in model.things.iter_mut() {
    thing.update();
  }

}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    let f = app.elapsed_frames();
    if f == 1 {
      draw.background().color(BLACK);
    }
    for thing in model.things.iter() {
      thing.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
    
}
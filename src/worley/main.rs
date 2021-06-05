use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const ANGLE_STEP: f32 = PI / 64.0;
const POS_STEP: f32 = 2.0;

struct Thing {
    pos: Point2,
    n: u64,
    angle: f32,
    points: Vec<Point2>
}
impl Thing {
    pub fn new() -> Self {
        Self {
            pos: pt2(-400.0, 0.0),
            n: random::<u64>(),
            angle: 0.0,
            points: vec!()
        }
    }
    fn update(&mut self) {
        if self.n == 1 { return; }
        let (angle, n) = if self.n & 1 == 0 {
            (ANGLE_STEP, self.n / 2)
        } else {
            (-ANGLE_STEP, (self.n * 3 + 1) / 2)
        };
        self.angle += angle;
        let inc = pt2(POS_STEP * self.angle.cos(), POS_STEP * self.angle.sin());
        self.points.push(self.pos.clone());
        self.pos += inc;
        self.n = n;
    }
    fn draw(&self, draw: &Draw) {
        draw.polyline().stroke_weight(4.0).points(self.points.clone()).color(srgba(0.6423, 0.2232, 0.0023, 0.1));
    }
}

struct Model {
    things: Vec<Thing>
}

const S: i32 = 800;
const H_S: i32 = S / 2;
const SCL: i32 = 10;
const H_SCL: i32 = SCL / 2;
const S_SCL: i32 = S / SCL;

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(S as u32, S as u32)
        .view(view)
        .build()
        .unwrap();
    Model {
        things: (0..100).map(|_| Thing::new()).collect()
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    for thing in model.things.iter_mut() {
        thing.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);
    for thing in model.things.iter() {
        thing.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
    //app.set_loop_mode(LoopMode::loop_once());
}
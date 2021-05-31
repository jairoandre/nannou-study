use nannou::prelude::*;

const W: u32 = 16000;
const H: u32 = 9000;
const SCL: u32 = 20;
const W_SCL: u32 = W / SCL;
const H_SCL: u32 = H / SCL;
const H_W_SCL: i32 = W_SCL as i32 / 2;
const H_H_SCL: i32 = H_SCL as i32 / 2;
const CIRCLE:u32 = 600;
const POD_RADIUS: u32 = 400 / SCL;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn vec_len_squared(pt: Vector2) -> f32 {
  pt.x * pt.x + pt.y + pt.y
}

struct Pod {
    pos: Point2,
    vel: Vector2,
    target: Point2,
    target_idx: usize,
    total_targets: usize,
}
impl Pod {
    pub fn new(pos: Point2, vel: Point2) -> Self {
        Pod {
            pos,
            vel,
            target: pt2(0.0, 0.0),
            target_idx: 1,
            total_targets: 4,
        }
    }
    fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(from_cg_coords(self.pos.x, self.pos.y)).w_h(POD_RADIUS as f32, POD_RADIUS as f32).color(WHITE);
    }
    fn seek(&mut self) {
        let desired = (self.target - self.pos).normalize() * 100.0;
        let steering = (desired - self.vel).limit_magnitude(0.5); // divide by the mass
        self.vel = (self.vel + steering).limit_magnitude(100.0)
    }
    fn update(&mut self) {
        self.pos += self.vel * 0.5;
        let pt_dist = self.target - self.pos;
        if vec_len_squared(pt_dist) < 10000.0 {
            self.target_idx = (self.target_idx + 1) % self.total_targets;
            eprintln!("{}", self.target_idx);
        }
    }
}

struct Model {
    track: Vec<Point2>,
    pod: Pod,
}

fn from_cg_coords(x: f32, y: f32) -> Point2 {
    let x_scl = x / SCL as f32;
    let y_scl = y / SCL as f32;
    let r_x = x_scl - H_W_SCL as f32;
    let r_y = -y_scl + H_H_SCL as f32;
    pt2(r_x as f32, r_y as f32)
}

fn random_point() -> Point2 {
    let x = random_range(100, W - 100) as i32;
    let y = random_range(100, H - 100) as i32;
    pt2(x as f32, y as f32)
}

fn draw_circle(draw: &Draw, pt: &Point2, radius: u32) {
    let r = (radius / SCL) as f32;
    let cg_coords = from_cg_coords(pt.x, pt.y);
    draw.ellipse().xy(cg_coords).w_h(r, r).color(WHITE);
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(W_SCL, H_SCL)
        .view(view)
        .build()
        .unwrap();
    let mut track = vec![];
    for _i in 0..4 {
        let pt = random_point();
        track.push(pt);
    }

    let initial_pos = track[0].clone();

    Model {
        track,
        pod: Pod::new(initial_pos, pt2(0.0, 0.0)),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let pod = &mut model.pod;
    //let target = to_cg_coords(app.mouse.x, app.mouse.y);
    let target = model.track[pod.target_idx].clone();
    pod.target = target;
    pod.seek();
    pod.update();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);
    draw.rect().w_h(W_SCL as f32, H_SCL as f32).x_y(0.0, 0.0).color(ORANGE);
    for (idx, pt) in model.track.iter().enumerate() {
        draw_circle(&draw, &pt, CIRCLE);
        draw.text(&format!("{}", idx)).xy(from_cg_coords(pt.x, pt.y)).color(BLACK);
    }
    model.pod.draw(&draw);
    draw.to_frame(app, &frame).unwrap();
}
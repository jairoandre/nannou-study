use nannou::prelude::*;

const PERCEPTION: f32 = 50.0;

pub struct Boid {
  pos: Point2,
  vel: Vector2,
  radius: f32,
  boundary: f32,
  color: Hsva
}
impl Boid {
  pub fn new(radius: f32, screen_size: f32) -> Self {
    let boundary = screen_size / 2.0;
    let n = random::<f32>();
    let color = hsva((n * 332.23).sin(), (n * 223.23).sin() * 0.2 + 0.8, (n * 442.33).sin() * 0.1 + 0.9, 0.3);
    Self {
      pos: (random::<Point2>() - vec2(0.5, 0.5)) * screen_size,
      vel: (random::<Vector2>() - vec2(0.5, 0.5)) *  5.0,
      radius,
      boundary,
      color,
    }
  }
  pub fn apply_force(&mut self, force: Vector2) {
    self.vel = (self.vel + force * 0.05).limit_magnitude(2.5);
    if self.vel.magnitude() < 1.0 {
      self.vel = self.vel.with_magnitude(1.0);
    }
    
  }
  pub fn update(&mut self) {
    self.pos += self.vel;
    if self.pos.x.abs() > self.boundary {
      self.pos.x *= -1.0;
    }
    if self.pos.y.abs() > self.boundary {
      self.pos.y *= -1.0;
    }
  }
  pub fn draw(&self, draw: &Draw) {

    draw.ellipse().wh(vec2(self.radius, self.radius)).xy(self.pos).color(self.color);
    //draw.text(&format!("[{},{}]", self.pos.x.floor() as i32, self.pos.y.floor() as i32)).xy(pt2(self.pos.x, self.pos.y - 10.0)).color(RED);
  }
  fn get_sep(&self, boid: &Boid) -> Vector2 {
      let mut sep_vec = self.pos.clone() - boid.pos.clone();
      let sep_mag = sep_vec.magnitude();
      sep_vec = sep_vec * (1.0 - sep_mag / PERCEPTION);
      sep_vec
  }
  fn compute_steer(&self, boids: &[Boid]) -> (Vector2, Vector2, Vector2, f32) {
    let mut desired_vel: Vector2<f32> = Vector2::zero();
    let mut desired_pos: Vector2<f32> = Vector2::zero();
    let mut desired_sep: Vector2<f32> = Vector2::zero();
    let mut t = 0;
    for boid in boids.iter() {
      if (boid.pos - self.pos).magnitude() > PERCEPTION {
        continue;
      }
      desired_vel += boid.vel;
      desired_pos += boid.pos;
      desired_sep += self.get_sep(&boid);
      t += 1;
    }
    (desired_vel, desired_pos, desired_sep, t as f32)
  }
  pub fn get_steer(&self, head: &[Boid], tail: &[Boid], limits: (f32, f32, f32)) -> Vector2 {
    let (dh_vel, dh_pos, dh_sep, th) = self.compute_steer(head);
    let (dt_vel, dt_pos, dt_sep, tt) = self.compute_steer(tail);
    let t = th + tt;
    if t < 1.0 {
      return Vector2::zero();
    }
    let desired_vel = (dh_vel + dt_vel) / t;
    let desired_pos = (dh_pos + dt_pos) / t;
    let desired_sep = (dh_sep + dt_sep) / t;

    let steer_vel = desired_vel - self.vel;
    let steer_pos = (desired_pos - self.pos) - self.vel;
    let steer_sep = desired_sep - self.vel;

    steer_vel.limit_magnitude(limits.0) + steer_pos.limit_magnitude(limits.1) + steer_sep.limit_magnitude(limits.2)
  }
}
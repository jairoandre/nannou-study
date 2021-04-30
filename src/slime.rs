use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

const HASH_F: u32 = 2654435769;
const SIZE: u32 = 400;
const SIZE_F: f32 = SIZE as f32;
const SCL: f32 = 2.0;
const U_SIZE: u32 = SIZE * SCL as u32;
const H_SCL: f32 = SCL * 0.5;
const OFFSET: f32 = SIZE as f32 * H_SCL;
const MOVE_SPEED: f32 = 1.0;
const DT: f32 = 0.1;


fn hash(s: u32) -> f32 {
    let mut state = s;
    state ^= 2747636419;
    state *= HASH_F;
    state ^= state >> 16;
    state *= HASH_F;
    state ^= state >> 16;
    state *= HASH_F;
    state as f32 / u32::MAX as f32
}

struct Model {
    agents: Vec<Agent>
}

struct Agent {
    position: Vector2<f32>,
    angle: f32,
}
impl Agent {
    pub fn new(position: Vector2<f32>, angle: f32) -> Self {
        Agent {
            position,
            angle,
        }
    }

    fn update(&mut self) {
        let rand = hash(self.position.y as u32 * SIZE + self.position.x as u32);
        let a = self.angle * TAU;
        let direction = vec2(a.cos(), a.sin());
        let mut new_position = self.position + direction * MOVE_SPEED * DT;
        if new_position.x > SIZE_F - 1.0 || new_position.x < 0.0 || new_position.y > SIZE_F - 1.0 || new_position.y < 0.0 {
            new_position.x = clamp(new_position.x, 0.0, SIZE_F);
            new_position.y = clamp(new_position.y, 0.0, SIZE_F);
            self.angle = rand;
        }
        self.position = new_position;
    }
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(U_SIZE, U_SIZE)
        .view(view)
        .build()
        .unwrap();
    let mut agents: Vec<Agent> = Vec::new();
    for _i in 0..100 {
        //let x = SIZE_F * random::<f32>();
        //let y = SIZE_F * random::<f32>();
        let x = SIZE_F * 0.5;
        let y = SIZE_F * 0.5;
        agents.push(Agent::new(vec2(x, y), random::<f32>()));
    }
    Model {
        agents
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for agent in model.agents.iter_mut() {
        agent.update();
    }
}



fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect().x_y(0.0, 0.0).w(U_SIZE as f32).h(U_SIZE as f32).color(srgba(0.0, 0.0, 0.0, 0.01));
    for agent in model.agents.iter() {
        let pos = agent.position;
        draw.rect().x_y(pos.x * SCL - OFFSET + H_SCL, pos.y * SCL - OFFSET + H_SCL).w_h(SCL, SCL).color(WHITE);
    }
    draw.to_frame(app, &frame).unwrap()
}
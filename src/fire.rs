use nannou::prelude::*; 
use nannou::ui::prelude::*; 

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

const SIZE: f32 = 360.0;
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
    ui: Ui,
    ids: Ids,
    frequency: f32,
    r_phase: f32,
    g_phase: f32,
    b_phase: f32,
}

widget_ids! {
    struct Ids {
        frequency,
        r_phase,
        g_phase,
        b_phase,
    }
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
    fn decay(&mut self, another: u32) {
        if another == 0  { return; }
        self.intensity = another + 1;
    }
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::Wait);

    let mut ui = app.new_ui().build().unwrap();

    let ids = Ids::new(ui.widget_id_generator());

    let mut particles: Vec<Particle> = Vec::new();
    for i in 0..N as usize {
        let intensity = if i < SIZE_SCL as usize { 35 } else { 0 };
        particles.push(Particle::new(intensity))
    }
    let frequency = 0.57;
    let r_phase = 7.0;
    let g_phase = 1.5;
    let b_phase = 8.5;

    Model {
        particles,
        ui,
        ids,
        frequency,
        r_phase,
        g_phase,
        b_phase,
    }
}

fn intensity_to_color(intensity: u32, model: &Model) -> Rgb {
    let n = 360.0 * (intensity as f32 / 36.0);
    let i = deg_to_rad(n);
    let r = (i * model.frequency + model.r_phase).sin() * 0.5 + 0.5;
    let g = (i * model.frequency + model.g_phase).sin() * 0.5 + 0.5;
    let b = (i * model.frequency + model.b_phase).sin() * 0.5 + 0.5;
    srgb(r, g, b)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.frequency as f32, 0.1, 1.0)
        .top_left_with_margins(10.0, 20.0)
        .label(&format!("Frequency {}", model.frequency))
        .set(model.ids.frequency, ui)
    {
        model.frequency = value as f32;
    }
    for value in slider(model.r_phase as f32, 0.0, 10.0)
        .top_left_with_margins(40.0, 20.0)
        .label(&format!("R phase: {}", model.r_phase))
        .set(model.ids.r_phase, ui)
    {
        model.r_phase = value as f32;
    }
    for value in slider(model.g_phase as f32, 0.0, 10.0)
        .top_left_with_margins(70.0, 20.0)
        .label(&format!("G phase: {}", model.g_phase))
        .set(model.ids.g_phase, ui)
    {
        model.g_phase = value as f32;
    }
    for value in slider(model.b_phase as f32, 0.0, 10.0)
        .top_left_with_margins(100.0, 20.0)
        .label(&format!("B phase: {}", model.b_phase))
        .set(model.ids.b_phase, ui)
    {
        model.b_phase = value as f32;
    }

    let particles = &mut model.particles;

    for index in 0..N as usize {
        spread_fire(index, particles);
    }

}

fn spread_fire(index: usize, particles: &mut Vec<Particle>) {
    if index < SIZE_SCL as usize { return; }
    let down_index = index - SIZE_SCL as usize;
    let down_intensity = particles[down_index].intensity;
    particles[index].decay(down_intensity);
}


fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);
    for (idx, particle) in model.particles.iter().enumerate() {
        let (x, y) = idx_to_x_y(idx);
        //println!("{} {} {}", x, y, particle.intensity);
        draw.rect().x_y(x, y).w_h(SCL, SCL).color(intensity_to_color(particle.intensity, model));
    }
    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}
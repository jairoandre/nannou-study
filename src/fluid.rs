use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        //.simple_window(view)
        .run();
}

const N: usize = 100;
const HALF_N: f32 = N as f32 * 0.5;
const NN: usize = N * N;
const N_FLOAT: f32 = N as f32;
const ITER: usize = 4;
const SCL: f32 = 5.0;
const N_SCL: f32 = N as f32 * SCL;
const HALF_N_SCL: f32 = N_SCL * 0.5;
const HALF_SCL: f32 = SCL * 0.5;

struct Fluid {
    dt: f32,
    diff: f32,
    visc: f32,
    s: Vec<f32>,
    density: Vec<f32>,
    vx: Vec<f32>,
    vy: Vec<f32>,
    vx0: Vec<f32>,
    vy0: Vec<f32>,
}
impl Fluid {
    pub fn new(dt: f32, diff: f32, visc: f32) -> Self {
        Fluid {
            dt,
            diff,
            visc,
            s: vec![0.0f32; NN],
            density: vec![0.0f32; NN],
            vx: vec![0.0f32; NN],
            vy: vec![0.0f32; NN],
            vx0: vec![0.0f32; NN],
            vy0: vec![0.0f32; NN],
        }
    }

    fn add_density(&mut self, x: usize, y: usize, ammount: f32) {
        let index = x_y_to_index(x, y);
        self.density[index] += ammount;
    }

    fn add_velocity(&mut self, x: usize, y: usize, x_ammount: f32, y_ammount: f32) {
        let index = x_y_to_index(x, y);
        self.vx[index] += x_ammount;
        self.vy[index] += y_ammount;
    }

    fn step(&mut self) {
        let dt = self.dt;
        let visc = self.visc;
        let diff = self.diff;
        let vx = &mut self.vx;
        let vx0 = &mut self.vx0;
        let vy = &mut self.vy;
        let vy0 = &mut self.vy0;
        let s = &mut self.s;
        let density = &mut self.density;

        diffuse(1, vx0, vx, visc, dt);
        diffuse(2, vy0, vy, visc, dt);

        project(vx0, vy0, vx, vy);

        advect(1, vx, vx0, vx0, vy0, dt);
        advect(2, vy, vy0, vx0, vy0, dt);

        project(vx, vy, vx0, vy0);

        diffuse(0, s, density, diff, dt);
        advect(0, density, s, vx, vy, dt);
    }

    fn decay(&mut self) {
        let density = &mut self.density;
        for i in 0..N {
            let delta = density[i] - 50.0;
            density[i] = clamp(delta, 0.0, 100.0)
        }
    }
}

fn x_y_to_index(x: usize, y: usize) -> usize {
    let xc = clamp(x, 0, N -1);
    let yc = clamp(y, 0, N -1);
    xc + yc * N
}

fn set_bnd(b: usize, x: &mut Vec<f32>) {
    for i in 1..N-1 {
        x[x_y_to_index(i, 0)] = x[x_y_to_index(i, 1)] * (if b == 2 { -1.0 } else { 1.0 });
        x[x_y_to_index(i, N-1)] = x[x_y_to_index(i, N-2)] * (if b == 2 { -1.0 } else { 1.0 });
    }
    for j in 1..N-1 {
        x[x_y_to_index(0, j)] = x[x_y_to_index(1, j)] * (if b == 1 { -1.0 } else { 1.0 });
        x[x_y_to_index(N-1, j)] = x[x_y_to_index(N-2, j)] * (if b == 1 { -1.0 } else { 1.0 });
    }
    x[x_y_to_index(0, 0)] = 0.33 * (x[x_y_to_index(1, 0)] + x[x_y_to_index(0, 1)]);
    x[x_y_to_index(0, N-1)] = 0.33 * (x[x_y_to_index(1, N-1)] + x[x_y_to_index(0, N-2)]);
    x[x_y_to_index(N-1, 0)] = 0.33 * (x[x_y_to_index(N-2, 0)] + x[x_y_to_index(N-1, 1)]);
    x[x_y_to_index(N-1, N-1)] = 0.33 * (x[x_y_to_index(N-2, N-1)] + x[x_y_to_index(N-1, N-2)]);

}


fn linear_solve(b: usize, x: &mut Vec<f32>, x0: &mut Vec<f32>, a: f32, c: f32) {
    let c_recip = 1.0 / c;
    for _k in 0..ITER {
        for j in 1..N-1 {
            for i in 1..N-1 {
                let index = x_y_to_index(i, j);
                x[index] = (x0[index] + a * (
                    x[x_y_to_index(i + 1, j)] +
                    x[x_y_to_index(i - 1, j)] +
                    x[x_y_to_index(i, j + 1)] +
                    x[x_y_to_index(i, j - 1)]
                )) * c_recip;
            }
        }
        set_bnd(b, x);
    }
}

fn diffuse(b: usize, x: &mut Vec<f32>, x0: &mut Vec<f32>, diff: f32, dt: f32) {
    let n_minus_2 = (N - 2) as f32;
    let a = dt * diff * n_minus_2 * n_minus_2;
    linear_solve(b, x, x0, a, 1.0 + 6.0 * a);
}

fn project(vx: &mut Vec<f32>, vy: &mut Vec<f32>, p: &mut Vec<f32>, div: &mut Vec<f32>) {
    for j in 1..N-1 {
        for i in 1..N-1 {
            div[x_y_to_index(i, j)] = -0.5 * (
                vx[x_y_to_index(i + 1, j)] -
                vx[x_y_to_index(i - 1, j)] +
                vy[x_y_to_index(i, j + 1)] -
                vy[x_y_to_index(i, j - 1)]
            ) / N as f32;
            p[x_y_to_index(i, j)] = 0.0;
        }
    }
    set_bnd(0, div);
    set_bnd(0, p);
    linear_solve(0, p, div, 1.0, 6.0);
    for j in 1..N-1 {
        for i in 1..N-1 {
            vx[x_y_to_index(i, j)] -= 0.5 * (p[x_y_to_index(i + 1, j)] - p[x_y_to_index(i - 1, j)]) * N as f32;
            vy[x_y_to_index(i, j)] -= 0.5 * (p[x_y_to_index(i, j + 1)] - p[x_y_to_index(i, j - 1)]) * N as f32;
        }
    }
    set_bnd(1, vx);
    set_bnd(2, vy);
}

fn advect(b: usize, d: &mut Vec<f32>, d0: & Vec<f32>, vx: & Vec<f32>, vy: & Vec<f32>, dt: f32) {
    let (mut i0, mut i1, mut j0, mut j1): (f32, f32, f32, f32);

    let dtx = dt * (N - 2) as f32;
    let dty = dt * (N - 2) as f32;

    let (mut tmp1, mut tmp2,  mut x, mut y): (f32, f32, f32, f32);

    let (mut s1, mut s0, mut t1, mut t0): (f32, f32, f32,f32);

    for j in 1..N-1 {
        let j_float = j as f32;
        for i in 1..N-1 {
            let i_float = i as f32;
            tmp1 = dtx * vx[x_y_to_index(i, j)];
            tmp2 = dty * vy[x_y_to_index(i, j)];
            x = i_float - tmp1;
            y = j_float - tmp2;
            if x < 0.5 { x = 0.5; }
            if x > 0.5 + N_FLOAT { x = 0.5 + N_FLOAT; }
            i0 = x.floor();
            i1 = i0 + 1.0;
            if y < 0.5 { y = 0.5; }
            if y > 0.5 + N_FLOAT { y = 0.5 + N_FLOAT; }
            j0 = y.floor();
            j1 = j0 + 1.0;

            s1 = x - i0;
            s0 = 1.0 - s1;
            t1 = y - j0;
            t0 = 1.0 - t1;

            let i0i = i0 as usize;
            let i1i = i1 as usize;
            let j0i = j0 as usize;
            let j1i = j1 as usize;

            d[x_y_to_index(i, j)] =
                s0 * (t0 * d0[x_y_to_index(i0i, j0i)] + t1 * d0[x_y_to_index(i0i, j1i)]) +
                s1 * (t0 * d0[x_y_to_index(i1i, j0i)] + t1 * d0[x_y_to_index(i1i, j1i)]);

        }
    }
    set_bnd(b, d);
}

struct Model {
    fluid: Fluid
}

fn model(app: &App) -> Model {
    let s = N as f32 * SCL;
    let _window = app
        .new_window()
        .size(s as u32, s as u32)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_moved(mouse_moved)
        .build()
        .unwrap();

    Model {
        fluid: Fluid::new(0.1, 0.000001, 0.000001)
    }
}

fn mouse_pressed(app: &App, model: &mut Model, _button: MouseButton) {
    let x = HALF_N + app.mouse.x / SCL;
    let y = HALF_N + app.mouse.y / SCL;
    let fluid = &mut model.fluid;
    fluid.add_density(x as usize, y as usize, 0.0);
    fluid.add_velocity(x as usize, y as usize, (random::<f32>() - 0.5) * 100.0, (random::<f32>() - 0.5) * 100.0);
}


fn mouse_moved(app: &App, model: &mut Model, _dir: Vector2<f32>) {
    if app.mouse.buttons.left().is_down() {
        let x = HALF_N + app.mouse.x / SCL;
        let y = HALF_N + app.mouse.y / SCL;
        let fluid = &mut model.fluid;
        fluid.add_density(x as usize, y as usize, random::<f32>() * 100.0);
        fluid.add_velocity(x as usize, y as usize, (random::<f32>() - 0.5) * 10.0, (random::<f32>() - 0.5) * 10.0);
    }
    if app.mouse.buttons.right().is_down() {
        let fluid = &mut model.fluid;
        for i in 0..N {
            fluid.s[i] = 0.0;
            fluid.density[i] = 0.0;
            fluid.vx[i] = 0.0;
            fluid.vy[i] = 0.0;
            fluid.vx0[i] = 0.0;
            fluid.vy0[i] = 0.0;
        }
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let fluid = &mut model.fluid;
    fluid.decay();
    fluid.step();
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);
    let fluid = & model.fluid;
    for j in 0..N {
        for i in 0..N {
            let index = x_y_to_index(i, j);
            let density = fluid.density[index];
            let x = i as f32 * SCL - HALF_N_SCL + HALF_SCL;
            let y = j as f32 * SCL - HALF_N_SCL + HALF_SCL;
            draw.rect().x_y(x, y).w(SCL).h(SCL).color(srgba(1.0, 1.0, 1.0, density / 100.0));
        }
    }
    draw.to_frame(app, &frame).unwrap()
}
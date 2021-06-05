use nannou::prelude::*;
use nannou::ui::prelude::*;

mod boid;
use boid::Boid;

const SIZE: u32 = 900;
const N: u32 = 300;
const R: f32 = 5.0;

widget_ids! {
    struct Ids {
        title,
        vel_label,
        vel_slider,
        pos_label,
        pos_slider,
        sep_label,
        sep_slider,
    }
}
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
  ui: Ui,
  ids: Ids,
  main_window: WindowId,
  flock: Vec<Boid>,
  vel_f: f32,
  pos_f: f32,
  sep_f: f32,
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .mouse_pressed(mouse_pressed)
        .size(SIZE, SIZE)
        .view(view)
        .build()
        .unwrap();

    let ui_window = app.new_window()
        .title(app.exe_name().unwrap() + " controls")
        .size(300, 200)
        .view(ui_view)
        .event(ui_event)
        //.key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut ui = app.new_ui().window(ui_window).build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    ui.clear_with(nannou::ui::prelude::color::DARK_CHARCOAL);
    let mut theme = ui.theme_mut();
    theme.label_color = nannou::ui::prelude::color::WHITE;
    theme.shape_color = nannou::ui::prelude::color::CHARCOAL;

    let mut the_model = Model {
      ui,
      ids,
      main_window,
      flock: (0..N).map(|_| Boid::new(R, SIZE as f32)).collect(),
      vel_f: 2.0,
      pos_f: 3.0,
      sep_f: 4.0,
    };

    ui_event(&app, &mut the_model, WindowEvent::Focused);

    the_model
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
  model.flock = (0..N).map(|_| Boid::new(R, SIZE as f32)).collect();
}


fn update(_app: &App, model: &mut Model, _update: Update) {
  let l = model.flock.len();
  for idx in 0..l {
    let head = &model.flock[..idx];
    let tail = &model.flock[l-idx..];
    let steer = model.flock[idx].get_steer(head, tail, (model.vel_f, model.pos_f, model.sep_f));
    let boid = &mut model.flock[idx];
    boid.apply_force(steer);
  }
  for boid in model.flock.iter_mut() {
    boid.update();
  }
  //app.set_loop_mode(LoopMode::loop_once());
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    if app.elapsed_frames() == 1 {
      draw.background().color(BLACK);
    } else {
      draw.rect().wh(Vector2::one() * SIZE as f32).xy(Point2::zero()).color(srgba(0.0, 0.0, 0.0, 0.01));
    }
    //draw.background().color(BLACK);

    for boid in model.flock.iter() {
      boid.draw(&draw);
    }
    //draw.polyline().stroke_weight(1.0).points(vec!(pt2(-400.0, 0.0), pt2(400.0, 0.0))).color(GREEN);
    //draw.polyline().stroke_weight(1.0).points(vec!(pt2(0.0, -400.0), pt2(0.0, 400.0))).color(RED);
    draw.to_frame(app, &frame).unwrap();
}

fn ui_event(_app: &App, model: &mut Model, _event: WindowEvent) {
    let ui = &mut model.ui.set_widgets();

    // Control panel title
    widget::Text::new("Flock Boid Control")
        .top_left_with_margin(10.0)
        .w_h(300.0, 40.0)
        .font_size(24)
        .set(model.ids.title, ui);

    widget::Text::new("Velocity Force")
        .down_from(model.ids.title, 15.0)
        .w_h(125.0, 30.0)
        .set(model.ids.vel_label, ui);

    for value in widget::Slider::new(model.vel_f, 0.0, 5.0)
        .right_from(model.ids.vel_label, 10.0)
        .w_h(150.0, 30.0)
        .label(&model.vel_f.to_string())
        .set(model.ids.vel_slider, ui)
    {
        model.vel_f = value;
    }

    widget::Text::new("Position Force")
        .down_from(model.ids.vel_label, 10.00)
        .w_h(125.0, 30.0)
        .set(model.ids.pos_label, ui);

    for value in widget::Slider::new(model.pos_f, 0.0, 5.0)
        .right_from(model.ids.pos_label, 10.0)
        .w_h(150.0, 30.0)
        .label(&model.pos_f.to_string())
        .set(model.ids.pos_slider, ui)
    {
        model.pos_f = value;
    }

    widget::Text::new("Separation")
        .down_from(model.ids.pos_label, 10.0)
        .w_h(125.0, 30.0)
        .set(model.ids.sep_label, ui);

    // Motion slider
    for value in widget::Slider::new(model.sep_f, 0.0, 5.0)
        .right_from(model.ids.sep_label, 10.0)
        .w_h(150.0, 30.0)
        .label(&model.sep_f.to_string())
        .set(model.ids.sep_slider, ui)
    {
        model.sep_f = value;
    }
}

fn ui_view(app: &App, model: &Model, frame: Frame) {
    model.ui.draw_to_frame_if_changed(app, &frame).unwrap();
}
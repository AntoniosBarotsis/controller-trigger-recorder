#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(clippy::unwrap_used, unused_results)]

use std::{
  process,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
  },
  time::Duration,
};

use circular_queue::CircularQueue;
use eframe::egui;
use egui::{mutex::Mutex, Color32, Rgba, Vec2b, Visuals};
use egui_plot::{Line, Plot, PlotPoints};
use gilrs::Button::{LeftTrigger2, RightTrigger2};
use gilrs::Gilrs;

// TODO:
// Refresh rate arg/enum default
// Thicker lines
// Longer?
fn main() {
  let should_exit = Arc::new(AtomicBool::new(false));
  let should_exit_clone = should_exit.clone();

  let app = MyApp::new(should_exit.clone());

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_decorations(false)
      // TODO: Next 2 lines wont work for other resolutions
      .with_inner_size([1920.0, 180.0])
      .with_position([0.0, 1080.0 - 180.0])
      .with_always_on_top()
      .with_transparent(true),
    ..Default::default()
  };

  let data_left = app.left.clone();
  let data_right = app.right.clone();

  std::thread::spawn(move || {
    let mut gilrs = Gilrs::new().unwrap();

    let mut prev_left = 0.0;
    let mut prev_right = 0.0;

    loop {
      std::thread::sleep(Duration::from_millis(10));
      let _tmp = gilrs.next_event();
      // TODO: Handle no connected controllers
      let (_id, gamepad) = gilrs.gamepads().next().unwrap();
      let left = gamepad.button_code(LeftTrigger2).unwrap();
      let right = gamepad.button_code(RightTrigger2).unwrap();

      let point_left = gamepad
        .state()
        .button_data(left)
        .map_or(prev_left, gilrs::ev::state::ButtonData::value);

      let point_right = gamepad
        .state()
        .button_data(right)
        .map_or(prev_right, gilrs::ev::state::ButtonData::value);

      data_left.lock().push(f64::from(point_left));
      data_right.lock().push(f64::from(point_right));

      prev_left = point_left;
      prev_right = point_right;

      if should_exit_clone.load(Ordering::Relaxed) {
        println!("Exiting...");
        break;
      }
    }
  });

  ctrlc::set_handler(move || should_exit.clone().store(true, Ordering::Relaxed))
    .expect("Error setting Ctrl-C handler");

  eframe::run_native(
    "My egui App",
    options,
    Box::new(move |cc| {
      egui_extras::install_image_loaders(&cc.egui_ctx);
      Box::<MyApp>::from(app)
    }),
  )
  .unwrap();
}

struct MyApp {
  left: Arc<Mutex<CircularQueue<f64>>>,
  right: Arc<Mutex<CircularQueue<f64>>>,
  window_size: usize,
  should_exit: Arc<AtomicBool>,
}

impl MyApp {
  fn new(should_exit: Arc<AtomicBool>) -> Self {
    let window_size = 500;
    let mut queue = CircularQueue::with_capacity(window_size);

    for _ in 0..=queue.capacity() {
      queue.push(0.0);
    }

    Self {
      left: Arc::new(Mutex::new(queue.clone())),
      right: Arc::new(Mutex::new(queue)),
      window_size,
      should_exit,
    }
  }

  fn queue_to_points(&self, queue: &Arc<Mutex<CircularQueue<f64>>>) -> Vec<[f64; 2]> {
    #[allow(clippy::cast_precision_loss)]
    queue
      .lock()
      .clone()
      .iter()
      .enumerate()
      .map(|(index, v)| [(self.window_size - index - 1) as f64, (*v) * 100.0])
      .collect::<Vec<_>>()
  }
}

impl eframe::App for MyApp {
  fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
    Rgba::TRANSPARENT.to_array()
  }

  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if self.should_exit.load(Ordering::Relaxed) {
      process::exit(0);
    }

    let frame = egui::containers::Frame::central_panel(&ctx.style());
    let invis_formater = |_v, _i, _r: &_| String::new();

    egui::CentralPanel::default()
      .frame(frame.fill(egui::Color32::TRANSPARENT).inner_margin(0.0))
      .show(ctx, |ui| {
        #[allow(clippy::cast_precision_loss)]
        let plot = Plot::new("id_source")
          .clamp_grid(true)
          .x_axis_formatter(invis_formater)
          .y_axis_formatter(invis_formater)
          .include_x(0)
          .include_x((self.window_size + 10) as f64)
          .include_y(1)
          .include_y(100)
          .show_background(false)
          .show_grid(Vec2b::default())
          .allow_scroll(false)
          .allow_drag(false)
          .allow_zoom(false);

        let right = self.queue_to_points(&self.right);
        let left = self.queue_to_points(&self.left);

        plot.show(ui, |plot_ui| {
          plot_ui.line(Line::new(PlotPoints::from(right)).color(Color32::GREEN));
          plot_ui.line(Line::new(PlotPoints::from(left)).color(Color32::RED));
        });
      });

    ctx.request_repaint_after(Duration::from_millis(500));
  }
}

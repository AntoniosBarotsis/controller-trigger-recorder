#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(clippy::unwrap_used)]

use std::{sync::Arc, time::Duration};

use circular_queue::CircularQueue;
use eframe::egui;
use egui::{mutex::Mutex, Color32, Vec2b};
use egui_plot::{Line, Plot, PlotPoints};
use gilrs::Button::{LeftTrigger2, RightTrigger2};
use gilrs::EventType::ButtonChanged;
use gilrs::{Event, Gilrs};

fn main() {
  let app = MyApp::default();

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
      .with_inner_size([1920.0, 180.0])
      .with_always_on_top()
      .with_transparent(true),
    ..Default::default()
  };

  let data_left = app.left.clone();
  let data_right = app.right.clone();

  std::thread::spawn(move || -> ! {
    let mut gilrs = Gilrs::new().unwrap();
    for (_id, gamepad) in gilrs.gamepads() {
      println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut prev_left: Option<f32> = None;
    let mut prev_right: Option<f32> = None;
    loop {
      std::thread::sleep(Duration::from_millis(15));

      if let Some(Event {
        id: _,
        event,
        time: _,
      }) = gilrs.next_event()
      {
        if let ButtonChanged(LeftTrigger2, v, _code) = event {
          data_left.lock().push(f64::from(v));
          prev_left = Some(v);

          if let Some(prev) = prev_right {
            data_right.lock().push(f64::from(prev));
          }
        }
        if let ButtonChanged(RightTrigger2, v, _code) = event {
          data_right.lock().push(f64::from(v));
          prev_right = Some(v);

          if let Some(prev) = prev_left {
            data_left.lock().push(f64::from(prev));
          }
        }
      } else {
        if let Some(prev) = prev_left {
          data_left.lock().push(f64::from(prev));
        }
        if let Some(prev) = prev_right {
          data_right.lock().push(f64::from(prev));
        }
      }
    }
  });

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
}

impl Default for MyApp {
  fn default() -> Self {
    let window_size = 500;
    let mut queue = CircularQueue::with_capacity(window_size);

    for _ in 0..=queue.capacity() {
      queue.push(0.0);
    }

    Self {
      left: Arc::new(Mutex::new(queue.clone())),
      right: Arc::new(Mutex::new(queue)),
      window_size,
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let frame = egui::containers::Frame::central_panel(&ctx.style());

    egui::CentralPanel::default()
      .frame(frame.fill(egui::Color32::TRANSPARENT))
      .show(ctx, |ui| {
        let plot = Plot::new("id_source")
          .include_x(0)
          .include_x(500)
          .include_y(1)
          .include_y(100)
          .clamp_grid(true)
          .show_background(false)
          .show_grid(Vec2b::default())
          .allow_scroll(false)
          .allow_zoom(false);

        // TODO: RW Lock
        let right = self
          .right
          .lock()
          .clone()
          .iter()
          .enumerate()
          .map(|(index, v)| [(self.window_size - index - 1) as f64, (*v) * 100.0])
          .collect::<Vec<_>>();
        let left = self
          .left
          .lock()
          .clone()
          .iter()
          .enumerate()
          .map(|(index, v)| [(self.window_size - index - 1) as f64, (*v) * 100.0])
          .collect::<Vec<_>>();

        plot.show(ui, |plot_ui| {
          plot_ui.line(Line::new(PlotPoints::from(right)).color(Color32::GREEN));
          plot_ui.line(Line::new(PlotPoints::from(left)).color(Color32::RED));
        });
      });

    ctx.request_repaint_after(Duration::from_millis(500));
  }
}

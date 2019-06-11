use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::Time;
use crate::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

pub struct Game {
  x: i32,
  y: i32,
  scale: i32,
  color_hue: u8,
}

impl Game {
  pub fn new() -> Self {
    Self {
      x: DEFAULT_WINDOW_WIDTH as i32 / 2,
      y: DEFAULT_WINDOW_HEIGHT as i32 / 2,
      scale: 10,
      color_hue: 0,
    }
  }

  pub fn render(&self, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.set_draw_color(hsv_to_rgb(self.color_hue, 255, 255));
    canvas
      .fill_rect(Rect::new(
        self.x - self.scale.abs() / 2,
        self.y - self.scale.abs() / 2,
        self.scale.abs() as u32,
        self.scale.abs() as u32,
      ))
      .unwrap();
  }

  pub fn update(&mut self, delta_time: Time) {
    self.color_hue =
      ((f64::from(self.color_hue) + 256.0 * delta_time) % 256.0).floor() as u8;
  }

  pub fn handle_event(&mut self, event: Event) {
    match event {
      Event::MouseMotion { x, y, .. } => {
        self.x = x;
        self.y = y;
      }
      Event::MouseWheel { y, .. } => {
        self.scale += y;
      }
      _ => {}
    }
  }
}

#[allow(clippy::many_single_char_names)]
fn hsv_to_rgb(h: u8, s: u8, v: u8) -> Color {
  let (h, s, v): (f32, f32, f32) =
    (f32::from(h) / 255.0, f32::from(s) / 255.0, f32::from(v) / 255.0);

  let i = (h * 6.0).floor();
  let f = h * 6.0 - i;
  let p = v * (1.0 - s);
  let q = v * (1.0 - f * s);
  let t = v * (1.0 - (1.0 - f) * s);

  let (r, g, b) = match i as u8 % 6 {
    0 => (v, t, p),
    1 => (q, v, p),
    2 => (p, v, t),
    3 => (p, q, v),
    4 => (t, p, v),
    5 => (v, p, q),
    _ => unreachable!(),
  };

  Color::RGB(
    (r * 255.0).floor() as u8,
    (g * 255.0).floor() as u8,
    (b * 255.0).floor() as u8,
  )
}

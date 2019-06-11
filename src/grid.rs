use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::Time;

pub struct Grid {}

impl Grid {
  pub fn new() -> Self { Self {} }

  pub fn render(&self, canvas: &mut WindowCanvas, render_area: Rect) {
    canvas.set_draw_color(Color::RGB(240, 240, 240));
    canvas.fill_rect(render_area).unwrap();
  }

  pub fn update(&mut self, delta_time: Time) {}

  pub fn handle_event(&mut self, event: Event) {}
}

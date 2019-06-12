use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::math;
use crate::Time;

const CELL_BORDER_COLOR: Color = Color { r: 0x66, g: 0x66, b: 0x66, a: 0xFF };

pub struct Grid {
  cols: u8,
  rows: u8,
}

impl Grid {
  pub fn new(rows: u8, cols: u8) -> Self { Self { cols, rows } }

  pub fn render(&self, canvas: &mut WindowCanvas, render_area: Rect) {
    canvas.set_draw_color(CELL_BORDER_COLOR);

    let (offset_x, offset_y, scale) = math::best_fit_inside(
      render_area.width(),
      render_area.height(),
      u32::from(self.cols),
      u32::from(self.rows),
    );

    for col in 0..self.cols {
      for row in 0..self.rows {
        canvas
          .draw_rect(Rect::new(
            render_area.x()
              + (offset_x + f64::from(col) * scale).round() as i32,
            render_area.y()
              + (offset_y + f64::from(row) * scale).round() as i32,
            scale.round() as u32,
            scale.round() as u32,
          ))
          .unwrap();
      }
    }
  }

  pub fn update(&mut self, delta_time: Time) {}

  pub fn handle_event(&mut self, event: Event) {}
}

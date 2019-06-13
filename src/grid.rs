use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::math;
use crate::Time;

const CELL_BORDER_COLOR: Color = color!(0x66, 0x66, 0x66);
const CELL_COLORS: [Color; 12] = [
  color!(238, 170, 170),
  color!(221, 187, 153),
  color!(204, 204, 136),
  color!(187, 221, 153),
  color!(170, 238, 170),
  color!(153, 221, 187),
  color!(136, 204, 204),
  color!(153, 187, 221),
  color!(170, 170, 238),
  color!(187, 153, 221),
  color!(204, 136, 204),
  color!(221, 153, 187),
];

#[derive(Clone)]
pub struct Cell {
  color: u8,
}

pub struct Grid {
  cols: u8,
  rows: u8,
  cells: Vec<Option<Cell>>,
}

impl Grid {
  pub fn new(cols: u8, rows: u8) -> Self {
    use rand::distributions::{Distribution, Uniform};

    let cols_distribution = Uniform::from(0..cols);
    let rows_distribution = Uniform::from(0..rows);
    let colors_distribution = Uniform::from(0..CELL_COLORS.len() as u8);

    let mut cells = vec![None; cols as usize * rows as usize];
    let mut rng = rand::thread_rng();
    for _ in 0..16 {
      let row = rows_distribution.sample(&mut rng);
      let col = cols_distribution.sample(&mut rng);
      let color = colors_distribution.sample(&mut rng);
      cells[row as usize * cols as usize + col as usize] = Some(Cell { color });
    }

    // Self { cols, rows, cells: vec![None; cols as usize * rows as usize] }
    Self { cols, rows, cells }
  }

  pub fn render(&self, canvas: &mut WindowCanvas, render_area: Rect) {
    let (offset_x, offset_y, scale) = math::best_fit_inside(
      render_area.width(),
      render_area.height(),
      u32::from(self.cols),
      u32::from(self.rows),
    );

    for row in 0..self.rows {
      for col in 0..self.cols {
        let cell =
          &self.cells[row as usize * self.cols as usize + col as usize];

        let rect = Rect::new(
          render_area.x() + math::f_to_i(offset_x + f64::from(col) * scale),
          render_area.y() + math::f_to_i(offset_y + f64::from(row) * scale),
          math::f_to_u(scale),
          math::f_to_u(scale),
        );

        if let Some(Cell { color }) = cell {
          canvas.set_draw_color(CELL_COLORS[*color as usize]);
          canvas.fill_rect(rect).unwrap();
        } else {
          canvas.set_draw_color(CELL_BORDER_COLOR);
          canvas.draw_rect(rect).unwrap();
        }
      }
    }
  }

  pub fn update(&mut self, delta_time: Time) {}

  pub fn handle_event(&mut self, event: Event) {}
}

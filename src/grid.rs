use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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

  cells_rect: Rect,
  cell_size: f64,
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

    Self {
      cols,
      rows,
      cells,

      cells_rect: Rect::new(0, 0, 0, 0),
      cell_size: 0.0,
    }
  }

  pub fn calculate_layout(&mut self, bounding_box: Rect) {
    self.cells_rect = bounding_box;

    let (padding_x, padding_y, scale) = math::best_fit_inside(
      bounding_box.width(),
      bounding_box.height(),
      u32::from(self.cols),
      u32::from(self.rows),
    );

    self.cell_size = scale;

    self.cells_rect = Rect::new(
      bounding_box.x() + math::f_to_i(padding_x),
      bounding_box.y() + math::f_to_i(padding_y),
      math::f_to_u(f64::from(self.cols) * self.cell_size),
      math::f_to_u(f64::from(self.rows) * self.cell_size),
    );
  }

  pub fn render(&self, canvas: &mut WindowCanvas) {
    for row in 0..self.rows {
      for col in 0..self.cols {
        let cell =
          &self.cells[row as usize * self.cols as usize + col as usize];

        let rect = Rect::new(
          self.cells_rect.x() + math::f_to_i(f64::from(col) * self.cell_size),
          self.cells_rect.y() + math::f_to_i(f64::from(row) * self.cell_size),
          math::f_to_u(self.cell_size),
          math::f_to_u(self.cell_size),
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

  pub fn handle_event(&mut self, event: Event) {
    if let Event::MouseButtonDown {
      mouse_btn: MouseButton::Left, x, y, ..
    } = event
    {
      if self.cells_rect.contains_point(Point::new(x, y)) {
        let cell_x = (x - self.cells_rect.x()) / math::f_to_i(self.cell_size);
        let cell_y = (y - self.cells_rect.y()) / math::f_to_i(self.cell_size);
        if let Some(cell) =
          &self.cells[cell_y as usize * self.cols as usize + cell_x as usize]
        {
        }
      }
    }
  }
}

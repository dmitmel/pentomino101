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
  cols: usize,
  rows: usize,
  cells: Vec<Option<Cell>>,

  cells_rect: Rect,
  cell_size: f64,

  current_cell: Option<Cell>,
  cursor_pos: Point,
  click_offset: Point,
}

impl Grid {
  pub fn new(cols: usize, rows: usize) -> Self {
    use rand::distributions::{Distribution, Uniform};

    let cols_distribution = Uniform::from(0..cols);
    let rows_distribution = Uniform::from(0..rows);
    let colors_distribution = Uniform::from(0..CELL_COLORS.len() as u8);

    let mut cells = vec![None; cols * rows];
    let mut rng = rand::thread_rng();
    for _ in 0..16 {
      let row = rows_distribution.sample(&mut rng);
      let col = cols_distribution.sample(&mut rng);
      let color = colors_distribution.sample(&mut rng);
      cells[row * cols + col] = Some(Cell { color });
    }

    Self {
      cols,
      rows,
      cells,

      cells_rect: Rect::new(0, 0, 0, 0),
      cell_size: 0.0,

      current_cell: None,
      cursor_pos: Point::new(0, 0),
      click_offset: Point::new(0, 0),
    }
  }

  pub fn calculate_layout(&mut self, bounding_box: Rect) {
    self.cells_rect = bounding_box;

    let (padding_x, padding_y, scale) = math::best_fit_inside(
      bounding_box.width(),
      bounding_box.height(),
      self.cols as u32,
      self.rows as u32,
    );

    self.cell_size = scale;

    self.cells_rect = Rect::new(
      bounding_box.x() + math::f_to_i(padding_x),
      bounding_box.y() + math::f_to_i(padding_y),
      math::f_to_u(self.cols as f64 * self.cell_size),
      math::f_to_u(self.rows as f64 * self.cell_size),
    );
  }

  pub fn render(&self, canvas: &mut WindowCanvas) {
    for row in 0..self.rows {
      for col in 0..self.cols {
        let cell = &self.cells[row * self.cols + col];

        let rect = Rect::new(
          self.cells_rect.x() + math::f_to_i(col as f64 * self.cell_size),
          self.cells_rect.y() + math::f_to_i(row as f64 * self.cell_size),
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

    if let Some(Cell { color }) = self.current_cell {
      canvas.set_draw_color(CELL_COLORS[color as usize]);
      canvas
        .fill_rect(Rect::new(
          self.cursor_pos.x + self.click_offset.x,
          self.cursor_pos.y + self.click_offset.y,
          math::f_to_u(self.cell_size),
          math::f_to_u(self.cell_size),
        ))
        .unwrap();
    }
  }

  pub fn update(&mut self, delta_time: Time) {}

  pub fn handle_event(&mut self, event: Event) {
    match event {
      Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
        if self.cells_rect.contains_point(Point::new(x, y)) {
          let (col, row) = self.screen_to_grid_coords(x, y);
          let cell_index = row * self.cols + col;

          let (cell_x, cell_y) = (
            self.cells_rect.x() + math::f_to_i(col as f64 * self.cell_size),
            self.cells_rect.y() + math::f_to_i(row as f64 * self.cell_size),
          );
          self.click_offset = Point::new(cell_x - x, cell_y - y);

          if self.current_cell.is_none() {
            if let Some(cell) = self.cells[cell_index].take() {
              self.current_cell = Some(cell);
            }
          } else if self.cells[cell_index].is_none() {
            if let Some(current_cell) = self.current_cell.take() {
              self.cells[cell_index] = Some(current_cell);
            }
          }
        }
      }

      Event::MouseMotion { x, y, .. } => {
        self.cursor_pos = Point::new(x, y);
      }

      _ => {}
    }
  }

  fn screen_to_grid_coords(&self, x: i32, y: i32) -> (usize, usize) {
    let col = (x - self.cells_rect.x()) / math::f_to_i(self.cell_size);
    let row = (y - self.cells_rect.y()) / math::f_to_i(self.cell_size);
    (col as usize, row as usize)
  }
}

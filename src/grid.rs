use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::math;
use crate::Time;

const CELL_BORDER_COLOR: Color = color!(167, 172, 216);
const CELL_COLORS: [Color; 12] = [
  color!(244, 67, 54),
  color!(255, 152, 0),
  color!(255, 193, 7),
  color!(205, 220, 57),
  color!(139, 195, 74),
  color!(76, 175, 80),
  color!(0, 150, 136),
  color!(33, 150, 243),
  color!(63, 81, 181),
  color!(103, 58, 183),
  color!(156, 39, 176),
  color!(96, 125, 139),
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

  current_cell: Option<(Cell, Point)>,
}

impl Grid {
  pub fn new(cols: usize, rows: usize) -> Self {
    let mut cells = vec![None; cols * rows];

    #[rustfmt::skip]
    let square: Vec<u8> = vec![
      0,  0,  1,  1,  1,  1,  1,  2,
      0,  0,  0,  3,  3,  2,  2,  2,
      4,  4,  4,  4,  3,  3,  7,  2,
      5,  5,  4, 99, 99,  3,  7,  7,
      9,  5,  5, 99, 99,  6,  6,  7,
      9,  5, 10, 11, 11,  8,  6,  7,
      9, 10, 10, 10, 11,  8,  6,  6,
      9,  9, 10, 11, 11,  8,  8,  8,
    ];
    for (index, color) in square.into_iter().enumerate() {
      if (color as usize) < CELL_COLORS.len() {
        let col = index % 8;
        let row = index / 8;
        cells[col + cols * row] = Some(Cell { color });
      }
    }

    Self {
      cols,
      rows,
      cells,

      cells_rect: Rect::new(0, 0, 0, 0),
      cell_size: 0.0,

      current_cell: None,
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

    if let Some((Cell { color }, point)) = self.current_cell {
      canvas.set_draw_color(CELL_COLORS[color as usize]);
      canvas
        .fill_rect(Rect::new(
          point.x(),
          point.y(),
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

          let cell_pos = Point::new(
            self.cells_rect.x() + math::f_to_i(col as f64 * self.cell_size),
            self.cells_rect.y() + math::f_to_i(row as f64 * self.cell_size),
          );

          if self.current_cell.is_none() {
            if let Some(cell) = self.cells[cell_index].take() {
              self.current_cell = Some((cell, cell_pos));
            }
          } else if self.cells[cell_index].is_none() {
            if let Some((current_cell, _)) = self.current_cell.take() {
              self.cells[cell_index] = Some(current_cell);
            }
          }
        }
      }

      Event::MouseMotion { xrel, yrel, .. } => {
        if let Some((_, cell_pos)) = self.current_cell.as_mut() {
          *cell_pos += Point::new(xrel, yrel);
        }
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

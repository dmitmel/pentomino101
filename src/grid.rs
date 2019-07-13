use sdl2::event::Event;
use sdl2::keyboard::{Mod, Scancode};
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

#[derive(Debug, Clone)]
pub struct Cell {
  color: u8,
}

pub struct Grid {
  cols: usize,
  rows: usize,
  cells: Vec<Option<Cell>>,

  cells_rect: Rect,
  cell_size: f64,

  current_cells: Option<Vec<(Cell, Point)>>,

  cursor_pos: Point,
}

impl Grid {
  pub fn new(cols: usize, rows: usize) -> Self {
    let mut grid = Self {
      cols,
      rows,
      cells: vec![None; cols * rows],

      cells_rect: Rect::new(0, 0, 0, 0),
      cell_size: 0.0,

      current_cells: None,

      cursor_pos: Point::new(0, 0),
    };

    let picture = "
 0  1
00001
22111
2233
 23       B
 43      BB
 4377777BB
 44668999
 5468889AA
555668 9A
5      AA
"
    .to_string();
    let mut col = 5;
    let mut row = 5;
    for (index, color) in picture.trim_matches('\n').chars().enumerate() {
      let color_index = match color {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        '\n' => {
          col = 5;
          row += 1;
          continue;
        }
        _ => {
          col += 1;
          continue;
        }
      };
      *grid.cell_mut(col, row) = Some(Cell { color: color_index });
      col += 1;
    }

    grid
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
        let cell = self.cell(col, row);

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

    if let Some(current_cells) = &self.current_cells {
      for (Cell { color }, offset) in current_cells {
        canvas.set_draw_color(CELL_COLORS[*color as usize]);
        canvas
          .fill_rect(Rect::from_center(
            self.cursor_pos + *offset,
            math::f_to_u(self.cell_size),
            math::f_to_u(self.cell_size),
          ))
          .unwrap();
      }
    }
  }

  pub fn update(&mut self, delta_time: Time) {}

  pub fn handle_event(&mut self, event: Event) {
    match event {
      Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
        let click_point = Point::new(x, y);

        if self.cells_rect.contains_point(click_point) {
          let (clicked_col, clicked_row) =
            self.screen_to_grid_coords(click_point);
          let clicked_cell = self.cell(clicked_col, clicked_row).clone();

          if clicked_cell.is_some() && self.current_cells.is_none() {
            let clicked_cell_color = clicked_cell.unwrap().color;

            let mut current_cells = Vec::with_capacity(5);

            for row in 0..self.rows {
              for col in 0..self.cols {
                let cell_pos = Point::new(
                  self.cells_rect.x()
                    + math::f_to_i((col as f64 + 0.5) * self.cell_size),
                  self.cells_rect.y()
                    + math::f_to_i((row as f64 + 0.5) * self.cell_size),
                );
                let cell_offset = cell_pos - self.cursor_pos;

                let cell = self.cell_mut(col, row);
                if let Some(Cell { color }) = cell {
                  if *color == clicked_cell_color {
                    current_cells.push((cell.take().unwrap(), cell_offset));
                  }
                }
              }
            }

            self.current_cells = Some(current_cells);
          } else if clicked_cell.is_none() && self.current_cells.is_some() {
            let mut cells_can_be_placed = true;

            for (_, cell_offset) in self.current_cells.as_ref().unwrap() {
              let cell_pos = self.cursor_pos + *cell_offset;

              if !self.cells_rect.contains_point(cell_pos) {
                cells_can_be_placed = false;
                break;
              }

              let (cell_col, cell_row) = self.screen_to_grid_coords(cell_pos);
              let cell_on_grid = self.cell(cell_col, cell_row);
              if cell_on_grid.is_some() {
                cells_can_be_placed = false;
                break;
              }
            }

            if cells_can_be_placed {
              for (cell, cell_offset) in self.current_cells.take().unwrap() {
                let cell_pos = self.cursor_pos + cell_offset;
                let (cell_col, cell_row) = self.screen_to_grid_coords(cell_pos);
                *self.cell_mut(cell_col, cell_row) = Some(cell);
              }
            }
          }
        }
      }

      Event::MouseMotion { x, y, .. } => {
        self.cursor_pos = Point::new(x, y);
      }

      Event::KeyDown { scancode: Some(scancode), keymod, .. } => {
        if let Some(current_cells) = self.current_cells.as_mut() {
          for (_, cell_pos) in current_cells {
            *cell_pos = match (scancode, keymod) {
              (Scancode::R, Mod::NOMOD) => {
                Point::new(-cell_pos.y(), cell_pos.x())
              }
              (Scancode::R, Mod::LSHIFTMOD) | (Scancode::R, Mod::RSHIFTMOD) => {
                Point::new(cell_pos.y(), -cell_pos.x())
              }
              (Scancode::V, Mod::NOMOD) => {
                Point::new(-cell_pos.x(), cell_pos.y())
              }
              (Scancode::H, Mod::NOMOD) => {
                Point::new(cell_pos.x(), -cell_pos.y())
              }
              _ => *cell_pos,
            };
          }
        }
      }

      _ => {}
    }
  }

  fn cell(&self, col: usize, row: usize) -> &Option<Cell> {
    &self.cells[row * self.cols + col]
  }

  fn cell_mut(&mut self, col: usize, row: usize) -> &mut Option<Cell> {
    &mut self.cells[row * self.cols + col]
  }

  fn screen_to_grid_coords<P>(&self, point: P) -> (usize, usize)
  where
    P: Into<(i32, i32)>,
  {
    let (x, y) = point.into();
    let col = (x - self.cells_rect.x()) / math::f_to_i(self.cell_size);
    let row = (y - self.cells_rect.y()) / math::f_to_i(self.cell_size);
    (col as usize, row as usize)
  }
}

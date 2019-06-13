use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::grid::Grid;
use crate::math;
use crate::Time;

const GRID_MARGIN_PERCENT: f64 = 0.1;

pub struct Game {
  grid: Grid,
  grid_rect: Rect,
}

impl Game {
  pub fn new() -> Self {
    Self { grid: Grid::new(20, 10), grid_rect: Rect::new(0, 0, 0, 0) }
  }

  pub fn calculate_layout(&mut self, bounding_box: Rect) {
    let (mut grid_offset_x, mut grid_offset_y, mut grid_size) =
      math::best_fit_inside(bounding_box.width(), bounding_box.height(), 1, 1);

    let grid_margin = grid_size * GRID_MARGIN_PERCENT;
    grid_size -= grid_margin * 2.0;
    grid_offset_x += grid_margin;
    grid_offset_y += grid_margin;

    self.grid_rect = Rect::new(
      bounding_box.x() + math::f_to_i(grid_offset_x),
      bounding_box.y() + math::f_to_i(grid_offset_y),
      math::f_to_u(grid_size),
      math::f_to_u(grid_size),
    );

    self.grid.calculate_layout(self.grid_rect);
  }

  pub fn render(&self, canvas: &mut WindowCanvas) { self.grid.render(canvas); }

  pub fn update(&mut self, delta_time: Time) { self.grid.update(delta_time); }

  pub fn handle_event(&mut self, event: Event) {
    self.grid.handle_event(event);
  }
}

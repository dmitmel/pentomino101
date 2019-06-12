use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::grid::Grid;
use crate::math;
use crate::Time;

const GRID_MARGIN_PERCENT: f64 = 0.1;

pub struct Game {
  grid: Grid,
}

impl Game {
  pub fn new() -> Self { Self { grid: Grid::new(10, 20) } }

  pub fn render(&self, canvas: &mut WindowCanvas) {
    let (window_width, window_height) = canvas.window().size();

    let (mut grid_offset_x, mut grid_offset_y, mut grid_size) =
      math::best_fit_inside(window_width, window_height, 1, 1);

    let grid_margin = grid_size * GRID_MARGIN_PERCENT;
    grid_size -= grid_margin * 2.0;
    grid_offset_x += grid_margin;
    grid_offset_y += grid_margin;

    self.grid.render(
      canvas,
      Rect::new(
        grid_offset_x.round() as i32,
        grid_offset_y.round() as i32,
        grid_size.round() as u32,
        grid_size.round() as u32,
      ),
    );
  }

  pub fn update(&mut self, delta_time: Time) { self.grid.update(delta_time); }

  pub fn handle_event(&mut self, event: Event) {
    self.grid.handle_event(event);
  }
}

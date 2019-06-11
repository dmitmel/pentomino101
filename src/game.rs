use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use crate::grid::Grid;
use crate::Time;

const GRID_MARGIN_PERCENT: f64 = 0.1;

pub struct Game {
  grid: Grid,
}

impl Game {
  pub fn new() -> Self { Self { grid: Grid::new() } }

  pub fn render(&self, canvas: &mut WindowCanvas) {
    let (window_width, window_height) = canvas.window().size();

    let (mut grid_size, mut grid_offset_x, mut grid_offset_y) =
      if window_width >= window_height {
        // landscape
        (window_height, (window_width - window_height) / 2, 0)
      } else {
        // portrait
        (window_width, 0, (window_height - window_width) / 2)
      };
    let grid_margin =
      (f64::from(grid_size) * GRID_MARGIN_PERCENT).round() as u32;
    grid_size -= grid_margin * 2;
    grid_offset_x += grid_margin;
    grid_offset_y += grid_margin;

    self.grid.render(
      canvas,
      Rect::new(
        grid_offset_x as i32,
        grid_offset_y as i32,
        grid_size,
        grid_size,
      ),
    );
  }

  pub fn update(&mut self, delta_time: Time) { self.grid.update(delta_time); }

  pub fn handle_event(&mut self, event: Event) {
    self.grid.handle_event(event);
  }
}

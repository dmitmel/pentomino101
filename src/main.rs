#![feature(duration_float)]

extern crate nalgebra_glm as glm;
extern crate sdl2;

use std::time::Instant;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod game;
mod grid;
mod math;

type Time = f64;

const MAX_FPS: f64 = 60.0;
const SECONDS_PER_FRAME: Time = 1.0 / MAX_FPS;

const WINDOW_TITLE: &str = env!("CARGO_PKG_NAME");
const DEFAULT_WINDOW_WIDTH: u32 = 800;
const DEFAULT_WINDOW_HEIGHT: u32 = 600;

fn main() {
  let sdl_context = sdl2::init().unwrap();

  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem
    .window(WINDOW_TITLE, DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
    .resizable()
    .position_centered()
    .build()
    .unwrap();
  let mut canvas = window.into_canvas().build().unwrap();

  let mut event_pump = sdl_context.event_pump().unwrap();

  let mut game = game::Game::new();

  let mut prev_time = Instant::now();
  let mut update_lag: Time = 0.0;
  'game_loop: loop {
    let current_time = Instant::now();
    let delta_time: Time = (current_time - prev_time).as_secs_f64();

    if delta_time >= SECONDS_PER_FRAME {
      prev_time = current_time;
      update_lag += delta_time;

      while let Some(event) = event_pump.poll_event() {
        match event {
          Event::Quit { .. }
          | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
            break 'game_loop
          }
          _ => game.handle_event(event),
        }
      }

      while update_lag >= SECONDS_PER_FRAME {
        game.update(delta_time);
        update_lag -= SECONDS_PER_FRAME;
      }

      canvas.set_draw_color(Color::RGB(0, 0, 0));
      canvas.clear();
      game.render(&mut canvas);
      canvas.present();
    }
  }
}

pub fn best_fit_inside(
  a_width: u32,
  a_height: u32,
  b_width: u32,
  b_height: u32,
) -> (f64, f64, f64) {
  let a_width = f64::from(a_width);
  let a_height = f64::from(a_height);
  let b_width = f64::from(b_width);
  let b_height = f64::from(b_height);

  let (mut offset_x, mut offset_y) = (0.0, 0.0);
  let scale: f64;

  if a_width / a_height >= b_width / b_height {
    scale = a_height / b_height;
    offset_x = (a_width - b_width * scale) / 2.0;
  } else {
    scale = a_width / b_width;
    offset_y = (a_height - b_height * scale) / 2.0;
  }

  (offset_x, offset_y, scale)
}

use crate::*;

#[test]
fn test_scaling_zeroes() {
    let (sx, sy) = scale(0, 0);
    assert_eq!(sx, MIN_X);
    assert_eq!(sy, MIN_Y);
}

#[test]
fn test_scaling_max_values() {
    let (sx, sy) = scale(WIDTH - 1, HEIGHT - 1);
    assert_relative_eq!(sx, MAX_X);
    assert_relative_eq!(sy, MAX_Y);
}

#[test]
fn test_scaling_intensity() {
  let mut iterations = 1000;
  assert_eq!(255, scale_intensity(iterations));

  iterations = 0;
  assert_eq!(0, scale_intensity(iterations));

  iterations = 500;
  assert_eq!(127, scale_intensity(iterations));
}
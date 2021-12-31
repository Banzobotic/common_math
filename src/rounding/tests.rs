#[cfg(test)]
use super::*;

#[test]
fn test_round() {
    assert_eq!(round(123.456_f64, 2), 123.46_f64);
    assert_eq!(round(123.456_f64, 0), 123_f64);
    assert_eq!(round(123.456_f32, 2), 123.46_f32);
}

#[test]
fn test_round_zeros() {
    assert_eq!(round_zeros(123.456_f64, 1), 120_f64);
    assert_eq!(round_zeros(123.456_f64, 0), 123_f64);
    assert_eq!(round_zeros(123_i32, 2), 100_i32);
}

#[test]
fn test_ceil() {
    assert_eq!(ceil(123.454_f64, 2), 123.46_f64);
    assert_eq!(ceil(123.456_f64, 0), 124_f64);
    assert_eq!(ceil(123.454_f32, 2), 123.46_f32);
}

#[test]
fn test_ceil_zeros() {
    assert_eq!(ceil_zeros(123.456_f64, 1), 130_f64);
    assert_eq!(ceil_zeros(123.456_f64, 0), 124_f64);
    assert_eq!(ceil_zeros(123_i32, 2), 200_i32);
}

#[test]
fn test_floor() {
    assert_eq!(floor(123.456_f64, 2), 123.45_f64);
    assert_eq!(floor(123.456_f64, 0), 123_f64);
    assert_eq!(floor(123.454_f32, 2), 123.45_f32);
}

#[test]
fn test_floor_zeros() {
    assert_eq!(floor_zeros(123.456_f64, 1), 120_f64);
    assert_eq!(floor_zeros(123.654_f64, 0), 123_f64);
    assert_eq!(floor_zeros(156_i32, 2), 100_i32);
}

#[test]
fn test_round_sf() {
    assert_eq!(round_sf(123456_f64, 4), 123500_f64);
    assert_eq!(round_sf(123.456_f64, 2), 120_f64);
    assert_eq!(round_sf(123.456_f32, 3), 123_f32);
}

#[cfg(test)]
use super::*;

#[test]
fn test_round() {
    assert_eq!(round(123.456_f64, 2), 123.46_f64);
    assert_eq!(round(123.456_f64, 0), 123_f64);
    assert_eq!(round(123.456_f32, 2), 123.46_f32);
    assert_eq!(round(-123.456_f64, 1), -123.5);
    assert_eq!(round(123_f64, 2), 123_f64);
}

#[test]
fn test_round_zeros() {
    assert_eq!(round_zeros(123.456_f64, 1), 120_f64);
    assert_eq!(round_zeros(123.456_f64, 0), 123_f64);
    assert_eq!(round_zeros(123_i32, 2), 100_i32);
    assert_eq!(round_zeros(12345_u64, 1), 12350_u64);
}

#[test]
fn test_ceil() {
    assert_eq!(ceil(123.454_f64, 2), 123.46_f64);
    assert_eq!(ceil(123.456_f64, 0), 124_f64);
    assert_eq!(ceil(123.454_f32, 2), 123.46_f32);
    assert_eq!(ceil(-123.456_f64, 1), -123.4_f64);
    assert_eq!(ceil(123_f64, 2), 123_f64);
}

#[test]
fn test_ceil_zeros() {
    assert_eq!(ceil_zeros(123.456_f64, 1), 130_f64);
    assert_eq!(ceil_zeros(123.456_f64, 0), 124_f64);
    assert_eq!(ceil_zeros(123_i32, 2), 200_i32);
    assert_eq!(ceil_zeros(123453789_u64, 4), 123460000);
    assert_eq!(ceil_zeros(12345_u32, 0), 12345_u32);
    assert_eq!(ceil_zeros(-12645_i32, 3), -12000_i32);
    // assert_eq!(ceil_zeros(-12345_i128, 3), -12000_i128);
}

#[test]
fn test_floor() {
    assert_eq!(floor(123.456_f64, 2), 123.45_f64);
    assert_eq!(floor(123.456_f64, 0), 123_f64);
    assert_eq!(floor(123.454_f32, 2), 123.45_f32);
    assert_eq!(floor(-123.426_f64, 1), -123.5);
    assert_eq!(floor(123_f64, 2), 123_f64);
}

#[test]
fn test_floor_zeros() {
    assert_eq!(floor_zeros(123.456_f64, 1), 120_f64);
    assert_eq!(floor_zeros(123.654_f64, 0), 123_f64);
    assert_eq!(floor_zeros(156_i32, 2), 100_i32);
    assert_eq!(floor_zeros(-12345_i64, 3), -13000_i64);
    // assert_eq!(round_zeros(123456789_i128, 4), 123450000);
}

#[test]
fn test_round_sf() {
    assert_eq!(round_sf(123456_f64, 4), 123500_f64);
    assert_eq!(round_sf(123.456_f64, 2), 120_f64);
    assert_eq!(round_sf(123.456_f32, 3), 123_f32);
    assert_eq!(round_sf(12345_i32, 3), 12300_i32);
    assert_eq!(round_sf(123456789_u64, 5), 123460000_u64);
    assert_eq!(round_sf(-123456_i64, 2), -120000_i64)
}

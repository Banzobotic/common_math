/// Returns the number rounded to a given number of decimal places
///
/// # Examples
///
/// ```
/// use common_math::rounding::round;
///
/// assert_eq!(round(123.456_f64, 2), 123.46_f64);
/// assert_eq!(round(123.456_f64, 0), 123_f64);
/// assert_eq!(round(123.456_f32, 2), 123.46_f64);
/// ```
#[inline]
pub fn round<T: Float>(number: T, decimal_places: u32) -> T {
    let power = 10_f64.powi(decimal_places as i32);
    number.apply_round(power)
}

/// Returns the number rounded to a given number of zeros
///
/// # Examples
///
/// ```
/// use common_math::rounding::round_zeros;
///
/// assert_eq!(round_zeros(123.456_f64, 1), 120_f64);
/// assert_eq!(round_zeros(123.456_f64, 0), 123_f64);
/// assert_eq!(round_zeros(123_i32, 2), 100_i32);
/// ```
#[inline]
pub fn round_zeros<T: Roundable>(number: T, zeros: u32) -> T {
    let power = 10_f64.powi(-(zeros as i32));
    number.apply_round_zeros(power)
}

/// Returns the number rounded up to a given number of decimal places
///
/// # Examples
///
/// ```
/// use common_math::rounding::ceil;
///
/// assert_eq!(ceil(123.454_f64, 2), 123.46_f64);
/// assert_eq!(ceil(123.456_f64, 0), 124_f64);
/// assert_eq!(ceil(123.454_f32, 2), 123.46_f32);
/// ```
#[inline]
pub fn ceil<T: Float>(number: T, decimal_places: u32) -> T {
    let power = 10_f64.powi(decimal_places as i32);
    number.apply_ceil(power)
}

/// Returns the number rounded up to a given number of zeros
///
/// # Examples
///
/// ```
/// use common_math::rounding::ceil_zeros;
///
/// assert_eq!(ceil_zeros(123.456_f64, 1), 130_f64);
/// assert_eq!(ceil_zeros(123.456_f64, 0), 124_f64);
/// assert_eq!(ceil_zeros(123_i32, 2), 200_i32);
/// ```
#[inline]
pub fn ceil_zeros<T: Roundable>(number: T, zeros: u32) -> T {
    let power = 10_f64.powi(-(zeros as i32));
    number.apply_ceil_zeros(power)
}

/// Returns the number rounded down to a given number of decimal places
///
/// # Examples
///
/// ```
/// use common_math::rounding::floor;
///
/// assert_eq!(floor(123.456_f64, 2), 123.45_f64);
/// assert_eq!(floor(123.456_f64, 0), 123_f64);
/// assert_eq!(floor(123.454_f32, 2), 123.454_f32);
/// ```
#[inline]
pub fn floor<T: Float>(number: T, decimal_places: u32) -> T {
    let power = 10_f64.powi(decimal_places as i32);
    number.apply_floor(power)
}

/// Returns the number rounded down to a given number of zeros
///
/// # Examples
///
/// ```
/// use common_math::rounding::floor_zeros;
///
/// assert_eq!(floor_zeros(123.456_f64, 1), 120_f64);
/// assert_eq!(floor_zeros(123.654_f64, 0), 123_f64);
/// assert_eq!(floor_zeros(156_i32, 2), 100_i32);
/// ```
#[inline]
pub fn floor_zeros<T: Roundable>(number: T, zeros: u32) -> T {
    let power = 10_f64.powi(-(zeros as i32));
    number.apply_floor_zeros(power)
}

/// Returns the number rounded to a given number of significant figures
///
/// # Examples
///
/// ```
/// use common_math::rounding::round_sf;
///
/// assert_eq!(round_sf(123456_f64, 4), 123500_f64);
/// assert_eq!(round_sf(123.456_f64, 2), 120_f64);
/// assert_eq!(round_sf(123.456_f32, 3), 123_f32);
/// ```
#[inline]
pub fn round_sf<T>(number: T, sig_figs: u32) -> T
where
    T: Float + std::cmp::PartialEq + Roundable + Copy,
{
    let s = number.private_to_string();

    // Run first section if no fractional part, run second section if there is
    if number == number.private_trunc() {
        // Get number of digits
        let digits = s.len() as u32;

        // Round to number of significant figures
        round_zeros(number, digits - sig_figs)
    } else {
        // Create a vector with the digits before and after the decimal place
        let sides: Vec<&str> = s.split('.').collect();

        // Find number of digits in the number
        let mut digits: i32 = 0;

        if sides[0] != "0" {
            digits += sides[0].len() as i32;
        }
        digits += sides[1].len() as i32;

        // Round to number of significant figures
        let power = 10_f64.powi(sides[1].len() as i32 - digits + sig_figs as i32);
        number.apply_round(power)
    }
}

pub trait Float {
    fn apply_round(self, power: f64) -> Self;
    fn apply_ceil(self, power: f64) -> Self;
    fn apply_floor(self, power: f64) -> Self;
    fn private_trunc(self) -> Self;
    fn private_to_string(self) -> String;
}

impl Float for f32 {
    fn apply_round(self, power: f64) -> f32 {
        (self * power as f32).round() / power as f32
    }

    fn apply_ceil(self, power: f64) -> f32 {
        (self * power as f32).ceil() / power as f32
    }

    fn apply_floor(self, power: f64) -> f32 {
        (self * power as f32).floor() / power as f32
    }

    fn private_trunc(self) -> f32 {
        self.trunc()
    }

    fn private_to_string(self) -> String {
        self.to_string()
    }
}

impl Float for f64 {
    fn apply_round(self, power: f64) -> f64 {
        (self * power).round() / power
    }

    fn apply_ceil(self, power: f64) -> f64 {
        (self * power).ceil() / power
    }

    fn apply_floor(self, power: f64) -> f64 {
        (self * power).floor() / power
    }

    fn private_trunc(self) -> f64 {
        self.trunc()
    }

    fn private_to_string(self) -> String {
        self.to_string()
    }
}

pub trait Roundable {
    fn apply_round_zeros(self, power: f64) -> Self;
    fn apply_ceil_zeros(self, power: f64) -> Self;
    fn apply_floor_zeros(self, power: f64) -> Self;
}

impl Roundable for f32 {
    fn apply_round_zeros(self, power: f64) -> f32 {
        (self * power as f32).round() / power as f32
    }

    fn apply_ceil_zeros(self, power: f64) -> f32 {
        (self * power as f32).ceil() / power as f32
    }

    fn apply_floor_zeros(self, power: f64) -> f32 {
        (self * power as f32).floor() / power as f32
    }
}

impl Roundable for f64 {
    fn apply_round_zeros(self, power: f64) -> f64 {
        (self * power).round() / power
    }

    fn apply_ceil_zeros(self, power: f64) -> f64 {
        (self * power).ceil() / power
    }

    fn apply_floor_zeros(self, power: f64) -> f64 {
        (self * power).floor() / power
    }
}

impl Roundable for i8 {
    fn apply_round_zeros(self, power: f64) -> i8 {
        ((self as f64 * power).round() / power) as i8
    }

    fn apply_ceil_zeros(self, power: f64) -> i8 {
        ((self as f64 * power).ceil() / power) as i8
    }

    fn apply_floor_zeros(self, power: f64) -> i8 {
        ((self as f64 * power).floor() / power) as i8
    }
}

impl Roundable for i16 {
    fn apply_round_zeros(self, power: f64) -> i16 {
        ((self as f64 * power).round() / power) as i16
    }

    fn apply_ceil_zeros(self, power: f64) -> i16 {
        ((self as f64 * power).ceil() / power) as i16
    }

    fn apply_floor_zeros(self, power: f64) -> i16 {
        ((self as f64 * power).floor() / power) as i16
    }
}

impl Roundable for i32 {
    fn apply_round_zeros(self, power: f64) -> i32 {
        ((self as f64 * power).round() / power) as i32
    }

    fn apply_ceil_zeros(self, power: f64) -> i32 {
        ((self as f64 * power).ceil() / power) as i32
    }

    fn apply_floor_zeros(self, power: f64) -> i32 {
        ((self as f64 * power).floor() / power) as i32
    }
}

impl Roundable for i64 {
    fn apply_round_zeros(self, power: f64) -> i64 {
        ((self as f64 * power).round() / power) as i64
    }

    fn apply_ceil_zeros(self, power: f64) -> i64 {
        ((self as f64 * power).ceil() / power) as i64
    }

    fn apply_floor_zeros(self, power: f64) -> i64 {
        ((self as f64 * power).floor() / power) as i64
    }
}

impl Roundable for u8 {
    fn apply_round_zeros(self, power: f64) -> u8 {
        ((self as f64 * power).round() / power) as u8
    }

    fn apply_ceil_zeros(self, power: f64) -> u8 {
        ((self as f64 * power).ceil() / power) as u8
    }

    fn apply_floor_zeros(self, power: f64) -> u8 {
        ((self as f64 * power).floor() / power) as u8
    }
}

impl Roundable for u16 {
    fn apply_round_zeros(self, power: f64) -> u16 {
        ((self as f64 * power).round() / power) as u16
    }

    fn apply_ceil_zeros(self, power: f64) -> u16 {
        ((self as f64 * power).ceil() / power) as u16
    }

    fn apply_floor_zeros(self, power: f64) -> u16 {
        ((self as f64 * power).floor() / power) as u16
    }
}

impl Roundable for u32 {
    fn apply_round_zeros(self, power: f64) -> u32 {
        ((self as f64 * power).round() / power) as u32
    }

    fn apply_ceil_zeros(self, power: f64) -> u32 {
        ((self as f64 * power).ceil() / power) as u32
    }

    fn apply_floor_zeros(self, power: f64) -> u32 {
        ((self as f64 * power).floor() / power) as u32
    }
}

impl Roundable for u64 {
    fn apply_round_zeros(self, power: f64) -> u64 {
        ((self as f64 * power).round() / power) as u64
    }

    fn apply_ceil_zeros(self, power: f64) -> u64 {
        ((self as f64 * power).ceil() / power) as u64
    }

    fn apply_floor_zeros(self, power: f64) -> u64 {
        ((self as f64 * power).floor() / power) as u64
    }
}

mod tests;

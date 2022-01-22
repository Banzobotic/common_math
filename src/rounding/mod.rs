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
    let power = 10_f64.powi(zeros as i32);
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
    let power = 10_f64.powi(zeros as i32);
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
    let power = 10_f64.powi(zeros as i32);
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
pub fn round_sf<T: Roundable>(number: T, sig_figs: u32) -> T
{
    // Get number of digits
    let digits: i32 = number.get_digits();

    // Round to number of significant figures
    let power = 10_f64.powi(digits - sig_figs as i32);
    number.apply_round_zeros(power)
}

pub trait Float {
    #[doc(hidden)]
    fn apply_round(self, power: f64) -> Self;
    #[doc(hidden)]
    fn apply_ceil(self, power: f64) -> Self;
    #[doc(hidden)]
    fn apply_floor(self, power: f64) -> Self;
}

impl Float for f32 {
    #[doc(hidden)]
    fn apply_round(self, power: f64) -> f32 {
        (self * power as f32).round() / power as f32
    }

    #[doc(hidden)]
    fn apply_ceil(self, power: f64) -> f32 {
        (self * power as f32).ceil() / power as f32
    }

    #[doc(hidden)]
    fn apply_floor(self, power: f64) -> f32 {
        (self * power as f32).floor() / power as f32
    }
}

impl Float for f64 {
    #[doc(hidden)]
    fn apply_round(self, power: f64) -> f64 {
        (self * power).round() / power
    }

    #[doc(hidden)]
    fn apply_ceil(self, power: f64) -> f64 {
        (self * power).ceil() / power
    }

    #[doc(hidden)]
    fn apply_floor(self, power: f64) -> f64 {
        (self * power).floor() / power
    }
}


pub trait Roundable {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> Self;
    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> Self;
    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> Self;
    #[doc(hidden)]
    fn get_digits(&self) -> i32;
}

impl Roundable for f32 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> f32 {
        (self / power as f32).round() * power as f32
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> f32 {
        (self / power as f32).ceil() * power as f32
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> f32 {
        (self / power as f32).floor() * power as f32
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        self.abs().log10().ceil() as i32
    }
}

impl Roundable for f64 {
    fn apply_round_zeros(self, power: f64) -> f64 {
        (self / power).round() * power
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> f64 {
        (self / power).ceil() * power
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> f64 {
        (self / power).floor() * power
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        self.abs().log10().ceil() as i32
    }
}

impl Roundable for i8 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> i8 {
        ((self as f64 / power).round() * power) as i8
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> i8 {
        ((self as f64 / power).ceil() * power) as i8
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> i8 {
        ((self as f64 / power).floor() * power) as i8
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

impl Roundable for i16 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> i16 {
        ((self as f64 / power).round() * power) as i16
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> i16 {
        ((self as f64 / power).ceil() * power) as i16
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> i16 {
        ((self as f64 / power).floor() * power) as i16
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

impl Roundable for i32 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> i32 {
        ((self as f64 / power).round() * power) as i32
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> i32 {
        ((self as f64 / power).ceil() * power) as i32
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> i32 {
        ((self as f64 / power).floor() * power) as i32
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

impl Roundable for i64 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> i64 {
        ((self as f64 / power).round() * power) as i64
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> i64 {
        ((self as f64 / power).ceil() * power) as i64
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> i64 {
        ((self as f64 / power).floor() * power) as i64
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

// impl Roundable for i128 {
//     fn apply_round_zeros(self, power: f64) -> Self {
//         todo!()
//     }
// 
//     fn apply_ceil_zeros(self, power: f64) -> Self {
//         todo!()
//     }
// 
//     fn apply_floor_zeros(self, power: f64) -> Self {
//         todo!()
//     }
// }

impl Roundable for u8 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> u8 {
        ((self as f64 / power).round() * power) as u8
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> u8 {
        ((self as f64 / power).ceil() * power) as u8
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> u8 {
        ((self as f64 / power).floor() * power) as u8
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

impl Roundable for u16 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> u16 {
        ((self as f64 / power).round() * power) as u16
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> u16 {
        ((self as f64 / power).ceil() * power) as u16
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> u16 {
        ((self as f64 / power).floor() * power) as u16
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

impl Roundable for u32 {
    fn apply_round_zeros(self, power: f64) -> u32 {
        ((self as f64 / power).round() * power) as u32
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> u32 {
        ((self as f64 / power).ceil() * power) as u32
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> u32 {
        ((self as f64 / power).floor() * power) as u32
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

impl Roundable for u64 {
    #[doc(hidden)]
    fn apply_round_zeros(self, power: f64) -> u64 {
        ((self as f64 / power).round() * power) as u64
    }

    #[doc(hidden)]
    fn apply_ceil_zeros(self, power: f64) -> u64 {
        ((self as f64 / power).ceil() * power) as u64
    }

    #[doc(hidden)]
    fn apply_floor_zeros(self, power: f64) -> u64 {
        ((self as f64 / power).floor() * power) as u64
    }

    #[doc(hidden)]
    fn get_digits(&self) -> i32 {
        (*self as f64).abs().log10().ceil() as i32
    }
}

// impl Roundable for u128 {
//     fn apply_round_zeros(self, power: f64) -> Self {
//         todo!()
//     }
// 
//     fn apply_ceil_zeros(self, power: f64) -> Self {
//         ((self / power as u128) + 1) * power as u128
//     }
// 
//     fn apply_floor_zeros(self, power: f64) -> Self {
//         let power = power as u128;
//         (self / power) * power
//     }
// }

mod tests;

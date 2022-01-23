/// Rounds the number to the given number of decimal places
///
/// # Examples
///
/// ```
/// use common_math::rounding::round;
///
/// assert_eq!(round(123.456_f64, 2), 123.46_f64);
/// assert_eq!(round(123.456_f64, 0), 123_f64);
/// assert_eq!(round(123.456_f32, 2), 123.46_f32);
/// ```
#[inline]
pub fn round<T: Float>(number: T, decimal_places: u32) -> T {
    number.round_dp(decimal_places)
}

/// Rounds the number to the given number of zeros
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
    number.round_zeros(zeros)
}

/// Rounds the number up to the given number of decimal places
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
    number.ceil_dp(decimal_places)
}

/// Rounds the number up to the given number of zeros
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
    number.ceil_zeros(zeros)
}

/// Rounds the number down to the given number of decimal places
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
    number.floor_dp(decimal_places)
}

/// Rounds the number down to the given number of zeros
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
    number.floor_zeros(zeros)
}

/// Rounds the number to the given number of significant figures
///
/// # Examples
///
/// ```
/// use common_math::rounding::round_sf;
///
/// assert_eq!(round_sf(123456_f64, 4), 123500_f64);
/// assert_eq!(round_sf(123.456_f64, 2), 120_f64);
/// assert_eq!(round_sf(123.456_f32, 4), 123.5_f32);
/// ```
#[inline]
pub fn round_sf<T: Roundable>(number: T, sig_figs: u32) -> T {
    number.round_sf(sig_figs)
}

/// Rounds the number up to the given number of significant figures
///
/// # Examples
///
/// ```
/// use common_math::rounding::ceil_sf;
///
/// assert_eq!(ceil_sf(123321_f64, 4), 123400_f64);
/// assert_eq!(ceil_sf(123.456_f64, 2), 130_f64);
/// assert_eq!(ceil_sf(123.321_f32, 4), 123.4_f32);
/// ```
#[inline]
pub fn ceil_sf<T: Roundable>(number: T, sig_figs: u32) -> T {
    number.ceil_sf(sig_figs)
}

/// Rounds the number down to the given number of significant figures
///
/// # Examples
///
/// ```
/// use common_math::rounding::floor_sf;
///
/// assert_eq!(floor_sf(123456_f64, 4), 123400_f64);
/// assert_eq!(floor_sf(656.323_f64, 2), 650_f64);
/// assert_eq!(floor_sf(123.456_f32, 4), 123.4_f32);
/// ```
#[inline]
pub fn floor_sf<T: Roundable>(number: T, sig_figs: u32) -> T {
    number.floor_sf(sig_figs)
}

pub trait Float {
    /// Rounds the number to the given number of decimal places
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123.456_f64.round_dp(2), 123.46_f64);
    /// assert_eq!(123.456_f64.round_dp(0), 123_f64);
    /// assert_eq!(123.456_f32.round_dp(2), 123.46_f32);
    /// ```
    fn round_dp(self, decimal_places: u32) -> Self;

    /// Rounds the number up to the given number of decimal places
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123.454_f64.ceil_dp(2), 123.46_f64);
    /// assert_eq!(123.456_f64.ceil_dp(0), 124_f64);
    /// assert_eq!(123.454_f32.ceil_dp(2), 123.46_f32);
    /// ```
    fn ceil_dp(self, decimal_places: u32) -> Self;

    /// Rounds the number down to the given number of decimal places
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123.456_f64.floor_dp(2), 123.45_f64);
    /// assert_eq!(123.456_f64.floor_dp(0), 123_f64);
    /// assert_eq!(123.454_f32.floor_dp(2), 123.454_f32);
    /// ```
    fn floor_dp(self, decimal_places: u32) -> Self;
}

impl Float for f32 {
    #[inline]
    fn round_dp(self, decimal_places: u32) -> f32 {
        let power = 10_f32.powi(decimal_places as i32);
        (self * power).round() / power
    }

    #[inline]
    fn ceil_dp(self, decimal_places: u32) -> f32 {
        let power = 10_f32.powi(decimal_places as i32);
        (self * power).ceil() / power
    }

    #[inline]
    fn floor_dp(self, decimal_places: u32) -> f32 {
        let power = 10_f32.powi(decimal_places as i32);
        (self * power).floor() / power
    }
}

impl Float for f64 {
    #[inline]
    fn round_dp(self, decimal_places: u32) -> f64 {
        let power = 10_f64.powi(decimal_places as i32);
        (self * power).round() / power
    }

    #[inline]
    fn ceil_dp(self, decimal_places: u32) -> f64 {
        let power = 10_f64.powi(decimal_places as i32);
        (self * power).ceil() / power
    }

    #[inline]
    fn floor_dp(self, decimal_places: u32) -> f64 {
        let power = 10_f64.powi(decimal_places as i32);
        (self * power).floor() / power
    }
}

pub trait Roundable {
    /// Rounds the number to the given number of zeros
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123.456_f64.round_zeros(1), 120_f64);
    /// assert_eq!(123.456_f64.round_zeros(0), 123_f64);
    /// assert_eq!(123_i32.round_zeros(2), 100_i32);
    /// ```
    fn round_zeros(self, zeros: u32) -> Self;

    /// Rounds the number up to the given number of zeros
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(ceil_zeros(123.456_f64, 1), 130_f64);
    /// assert_eq!(ceil_zeros(123.456_f64, 0), 124_f64);
    /// assert_eq!(ceil_zeros(123_i32, 2), 200_i32);
    /// ```
    fn ceil_zeros(self, zeros: u32) -> Self;

    /// Rounds the number down to the given number of zeros
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123.456_f64.floor_zeros(1), 120_f64);
    /// assert_eq!(123.654_f64.floor_zeros(0), 123_f64);
    /// assert_eq!(156_i32.floor_zeros(2), 100_i32);
    /// ```
    fn floor_zeros(self, zeros: u32) -> Self;

    /// Rounds the number to the given number of significant figures
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123456_f64.round_sf(4), 123500_f64);
    /// assert_eq!(123.456_f64.round_sf(2), 120_f64);
    /// assert_eq!(123.456_f32.round_sf(4), 123.5_f32);
    /// ```
    fn round_sf(self, sig_figs: u32) -> Self;

    /// Rounds the number up to the given number of significant figures
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123321_f64.ceil_sf(4), 123400_f64);
    /// assert_eq!(123.456_f64.ceil_sf(2), 130_f64);
    /// assert_eq!(123.321_f32.ceil_sf(4), 123.4_f32);
    /// ```
    fn ceil_sf(self, sig_figs: u32) -> Self;

    /// Rounds the number down to the given number of significant figures
    ///
    /// # Examples
    ///
    /// ```
    /// use common_math::rounding::*;
    ///
    /// assert_eq!(123456_f64.floor_sf(4), 123400_f64);
    /// assert_eq!(656.323_f64.floor_sf(2), 650_f64);
    /// assert_eq!(123.456_f32.floor_sf(4), 123.4_f32);
    /// ```
    fn floor_sf(self, sig_figs: u32) -> Self;
    
    #[doc(hidden)]
    fn get_digits(&self) -> u32;
}

impl Roundable for f32 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> f32 {
        let power = 10_f32.powi(zeros as i32);
        (self / power).round() * power
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> f32 {
        let power = 10_f32.powi(zeros as i32);
        (self / power).ceil() * power
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> f32 {
        let power = 10_f32.powi(zeros as i32);
        (self / power).floor() * power
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f32.powi(digits - sig_figs as i32);
        (self / power).round() * power
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f32.powi(digits - sig_figs as i32);
        (self / power).ceil() * power
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f32.powi(digits - sig_figs as i32);
        (self / power).floor() * power
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        self.abs().log10().ceil() as u32
    }
}

impl Roundable for f64 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> f64 {
        let power = 10_f64.powi(zeros as i32);
        (self / power).round() * power
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> f64 {
        let power = 10_f64.powi(zeros as i32);
        (self / power).ceil() * power
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> f64 {
        let power = 10_f64.powi(zeros as i32);
        (self / power).floor() * power
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        (self / power).round() * power
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        (self / power).ceil() * power
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        (self / power).floor() * power
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        self.abs().log10().ceil() as u32
    }
}

impl Roundable for i8 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> i8 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as i8
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> i8 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as i8
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> i8 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as i8
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as i8
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as i8
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as i8
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

impl Roundable for i16 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> i16 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as i16
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> i16 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as i16
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> i16 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as i16
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as i16
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as i16
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as i16
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

impl Roundable for i32 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> i32 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as i32
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> i32 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as i32
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> i32 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as i32
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as i32
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as i32
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as i32
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

impl Roundable for i64 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> i64 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as i64
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> i64 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as i64
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> i64 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as i64
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as i64
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as i64
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as i64
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

impl Roundable for u8 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> u8 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as u8
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> u8 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as u8
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> u8 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as u8
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as u8
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as u8
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as u8
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

impl Roundable for u16 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> u16 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as u16
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> u16 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as u16
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> u16 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as u16
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as u16
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as u16
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as u16
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

impl Roundable for u32 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> u32 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as u32
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> u32 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as u32
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> u32 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as u32
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as u32
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as u32
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as u32
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

impl Roundable for u64 {
    #[inline]
    fn round_zeros(self, zeros: u32) -> u64 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).round() * power) as u64
    }

    #[inline]
    fn ceil_zeros(self, zeros: u32) -> u64 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).ceil() * power) as u64
    }

    #[inline]
    fn floor_zeros(self, zeros: u32) -> u64 {
        let power = 10_f64.powi(zeros as i32);
        ((self as f64 / power).floor() * power) as u64
    }

    #[inline]
    fn round_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).round() * power) as u64
    }

    #[inline]
    fn ceil_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).ceil() * power) as u64
    }

    #[inline]
    fn floor_sf(self, sig_figs: u32) -> Self {
        let digits: i32 = self.get_digits() as i32;
        let power = 10_f64.powi(digits - sig_figs as i32);
        ((self as f64 / power).floor() * power) as u64
    }

    #[doc(hidden)]
    #[inline]
    fn get_digits(&self) -> u32 {
        (*self as f64).abs().log10().ceil() as u32
    }
}

mod tests;

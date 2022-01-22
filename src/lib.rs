// Copyright 2022 Andrew Twigg

//! Common Math is a library providing some common math functions 
//! not provided by the standard library
//! 
//! Currently this library only provides functions for rounding 
//! however more may be added in the future
//! 

/// Functions for rounding
/// 
/// Provides functions for rounding to a number of decimal places, zeros or significant figures
/// 
/// All functions will try to adhere to the same rounding rules as the standard library
/// 
/// ```rust
/// use common_math::rounding::*;
/// ```
pub mod rounding;

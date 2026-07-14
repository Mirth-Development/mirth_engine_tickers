
// Imports
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use std::fmt::Display;
use std::ops::{Add, Div, Rem, Sub};


pub trait CountValue:
Copy                    // CountValue types are integers, which means they're safe to copy.
+ Ord                   // CountValue types are integers, hence Ord is necessary for comparison.
+ Display               // Making it so values can be printed to the console.
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Rem<Output = Self>
+ Send                  // Needed for Bevy queries; also lets Counts move safely across threads.
+ Sync                  // Needed for Bevy queries; also lets Counts be shared safely across threads.
+ 'static               // Needed for Bevy queries; also enforces that CountValue types own their data, with no borrowed lifetimes.
{
    const MIN: Self;
    const MAX: Self;
    fn absolute(self)               -> Self;
    fn as_f64(self)                 -> f64;
    fn as_i8(self)                  -> i8;
    fn as_i64(self)                 -> i64;
    fn from_f64(value: f64)         -> Self;
    fn from_i32(val: i32)           -> Self;
}

impl CountValue for i8 {
    const MIN: Self                 = i8::MIN + 1;
    const MAX: Self                 = i8::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn as_f64(self)                 -> f64  { self as f64 }
    fn as_i8(self)                  -> i8   { self }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i8 }
    fn from_i32(value: i32)         -> Self { value as i8 }
}

impl CountValue for i16 {
    const MIN: Self                 = i16::MIN + 1;
    const MAX: Self                 = i16::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn as_f64(self)                 -> f64  { self as f64 }
    fn as_i8(self)                  -> i8   { self as i8 }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i16 }
    fn from_i32(value: i32)         -> Self { value as i16 }
}

impl CountValue for i32 {
    const MIN: Self                 = i32::MIN + 1;
    const MAX: Self                 = i32::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn as_f64(self)                 -> f64  { self as f64 }
    fn as_i8(self)                  -> i8   { self as i8 }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i32 }
    fn from_i32(value: i32)         -> Self { value }
}



#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "count_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "count_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub struct Count<V: CountValue> {
    start_value:        V,
    current_value:      V,
    end_value:          V,
    is_bound_by_start:  bool,
    is_bound_by_end:    bool,
}

impl<V: CountValue> Default for Count<V> {

    fn default() -> Self {
        Self {
            start_value:        V::from_i32(0),
            current_value:      V::from_i32(0),
            end_value:          V::MAX,
            is_bound_by_start:  true,
            is_bound_by_end:    true,
        }
    }
}

impl<V: CountValue> Count<V> {

    // ##################################### CONSTRUCTORS ######################################## //
    /// PANIC EVALUATION WILL HAVE TO ACCOUNT FOR WHICH BOUNDARIES ARE ACTIVE
    pub fn new(
        start_value:        V,
        current_value:      V,
        end_value:          V,
        is_bound_by_start:  bool,
        is_bound_by_end:    bool,
    ) -> Self {

        let min = start_value.min(end_value);
        let max = start_value.max(end_value);

        // Panic Evaluators
        check_if_value_is_within_range(start_value, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(end_value, V::MIN, V::MAX);

        Self {
            start_value,
            current_value,
            end_value,
            is_bound_by_start,
            is_bound_by_end,
        }
    }
    // ######################################################################################## //



    // ##################################### GETTERS ########################################## //

    ///
    #[inline]
    pub fn start_value(&self) -> V {
        self.start_value
    }

    ///
    #[inline]
    pub fn current_value(&self) -> V {
        self.current_value
    }

    ///
    #[inline]
    pub fn end_value(&self) -> V {
        self.end_value
    }

    ///
    #[inline]
    pub fn is_bound_by_start(&self) -> bool {
        self.is_bound_by_start
    }

    ///
    #[inline]
    pub fn is_bound_by_end(&self) -> bool {
        self.is_bound_by_end
    }

    ///
    #[inline]
    pub fn is_double_bounded(&self) -> bool {
        self.is_bound_by_start && self.is_bound_by_end
    }

    #[inline]
    pub fn digit(&self, place: i32) -> Option<i8> {

        // The divisor for place N is 10^(N-1).
        let divisor = match place {
            1  => V::from_i32(1),
            2  => V::from_i32(10),
            3  => V::from_i32(100),
            4  => V::from_i32(1_000),
            5  => V::from_i32(10_000),
            6  => V::from_i32(100_000),
            7  => V::from_i32(1_000_000),
            8  => V::from_i32(10_000_000),
            9  => V::from_i32(100_000_000),
            10 => V::from_i32(1_000_000_000),
            _  => return None, // out-of-range place
        };

        // Count supports negatives for current_value, must flip to positive for calculation.
        let value = self.current_value.absolute();

        // The ones place always exists; every other place requires current_value to reach it.
        if (place == 1) || (value >= divisor) {
            Some(((value / divisor) % V::from_i32(10)).as_i8())
        }
        else {
            None
        }
    }
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //

    ///
    #[inline]
    pub fn set_start_value(&mut self, value: V) {

    }

    ///
    #[inline]
    pub fn set_current_value(&mut self, value: V) {

    }

    ///
    #[inline]
    pub fn set_end_value(&mut self, value: V) {

    }

    ///
    #[inline]
    pub fn activate_start_boundary(&mut self) {

    }

    ///
    #[inline]
    pub fn deactivate_start_boundary(&mut self) {

    }

    ///
    #[inline]
    pub fn activate_end_boundary(&mut self) {

    }

    ///
    #[inline]
    pub fn deactivate_end_boundary(&mut self) {

    }

    ///
    #[inline]
    pub fn activate_boundaries(&mut self) {

    }

    ///
    #[inline]
    pub fn deactivate_boundaries(&mut self) {

    }
    // ######################################################################################## //



    // ################################### EQUALITY METHODS ##################################### //
    ///
    #[inline]
    pub fn is_current_at_start(&self) -> bool {
        self.current_value == self.start_value
    }

    ///
    #[inline]
    pub fn is_current_at_end(&self) -> bool {
        self.current_value == self.end_value
    }

    ///
    #[inline]
    pub fn is_start_at_end(&self) -> bool {
        self.start_value == self.end_value
    }
    // ######################################################################################## //



    // ################################# DIFFERENCE METHODS ################################### //
    ///
    pub fn difference_from_start(&self) -> i64 {
        let min: i64 = self.current_value.min(self.start_value).as_i64();
        let max: i64 = self.current_value.max(self.start_value).as_i64();
        max - min
    }

    ///
    pub fn difference_from_end(&self) -> i64 {
        let min: i64 = self.current_value.min(self.end_value).as_i64();
        let max: i64 = self.current_value.max(self.end_value).as_i64();
        max - min
    }

    ///
    pub fn difference_between_boundaries(&self) -> i64 {
        let min: i64 = self.start_value.min(self.end_value).as_i64();
        let max: i64 = self.start_value.max(self.end_value).as_i64();
        max - min
    }
    // ######################################################################################## //



    // ################################### SUM METHODS ######################################## //
    ///
    pub fn sum_to_start_value(&mut self, value: V) {

    }

    ///
    #[inline]
    pub fn sum_to_current_value(&mut self, value: V) {

    }

    ///
    pub fn sum_to_end_value(&mut self, value: V) {

    }
    // ########################################################################################## //



    // ################################### PERCENTAGE METHODS ################################### //
    ///
    pub fn percentage_completed(&self) -> Option<f64> {

        if self.start_value == self.end_value {
            return None;
        }

        let start: f64 = self.start_value.as_f64();
        let current: f64 = self.current_value.as_f64();
        let end: f64 = self.end_value.as_f64();

        let range_reciprocal: f64 = 1.0 / (end - start);

        Some((current - start) * range_reciprocal)
    }

    ///
    pub fn percentage_remaining(&self) -> Option<f64> {

        if self.start_value == self.end_value {
            return None;
        }

        let start: f64 = self.start_value.as_f64();
        let current: f64 = self.current_value.as_f64();
        let end: f64 = self.end_value.as_f64();

        let range_reciprocal: f64 = 1.0 / (end - start);

        Some((end - current) * range_reciprocal)
    }
    // ########################################################################################## //



    // ##################################### RESET METHODS ###################################### //
    ///
    #[inline]
    pub fn reset(&mut self) {
        self.current_value = self.start_value;
    }
    // ########################################################################################## //



    // ###################################### HELPER METHODS ######################################## //
    ///
    pub fn print_information(&self) {
        println!("START_VALUE: {}", self.start_value);
        println!("CURRENT_VALUE: {}", self.current_value);
        println!("END_VALUE: {}", self.end_value);
        println!("IS_BOUND_BY_START: {}", self.is_bound_by_start);
        println!("IS_BOUND_BY_END: {}", self.is_bound_by_end);
    }
    // ############################################################################################## //
}



// ##################################### PANIC FUNCTIONS ######################################## //
/// Checks if a value falls within the provided minimum and maximum range (inclusive), will `PANIC` if the value is outside the provided range.
/// If a `PANIC` were to occur, a printed message will be displayed to explain how to avoid the `PANIC`.
///
/// Accepts any type that implements [`PartialOrd`] and [`Display`], meaning
/// all numeric primitives, [`char`], [`String`], and [`&str`] are valid inputs.
///
/// #### Example
/// ```ignore
/// check_if_value_is_within_range(5, 1, 10);    // Passes
/// check_if_value_is_within_range(15, 1, 10);   // Panics
/// ```
fn check_if_value_is_within_range<T: PartialOrd + Display>(value: T, minimum: T, maximum: T) {
    assert!(
        value >= minimum && value <= maximum,
        "{}[TICKER PANIC]{} Count value must be between {} and {} (inclusive). Got {}.  You can avoid this panic by doing the following:
        1. Make sure your Count constructor has the current_value set to a number that is between start_value and end_value (inclusive).
        2. Make sure your Count constructor has the start_value set to a number that is between CountValue::MIN and CountValue::MAX (inclusive).
        3. Make sure your Count constructor has the end_value set to a number that is between CountValue::MIN and CountValue::MAX (inclusive).",
        "\x1b[31m", "\x1b[0m", minimum, maximum, value
    );
}
// ############################################################################################## //

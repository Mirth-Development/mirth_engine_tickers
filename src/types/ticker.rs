
// Imports
use bevy::prelude::*;

/// By themselves, tickers can be used to create simple timers.  Although they are best used in conjunction
/// as an inner element to a greater time structure to create some wicked tickety-tocking.
///
/// All fields of Ticker have getters, and only digit has no setter.
///
/// # TICKING LOOPS AT I8::MAX
/// Tickers don't stop ticking.  Once the next tick addition hits i8::MAX it will zero out current_value using to_zero().
/// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
/// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
/// **If you're okay with values from -128 to 127 for your timers, then feel free to go ham with Tickers.**  Otherwise,
/// I recommend the Chronolog structure.
#[derive(Component, Reflect, Debug)]
pub struct Ticker {
    start_value: i8,
    current_value: i8,
    digit: i8,              // Set to i8 to reduce typecasting throughout methods, less to process.  Won't hurt since i8 covers digits 0 - 9 and the logic in Ticker methods forces only 0 - 9 to appear.
    timer: Timer,
}

impl Default for Ticker {

    /// The default ticker counts up every second when its .tick method is used and all other fields start at 0.
    ///
    /// # TICKING LOOPS AT I8::MAX
    /// Tickers don't stop ticking.  Once the next tick addition hits i8::MAX it will zero out current_value using to_zero().
    /// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
    /// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
    /// **If you're okay with values from -128 to 127 for your timers, then feel free to go ham with Tickers.**  Otherwise,
    /// I recommend the Chronolog structure.
    fn default() -> Self {
        Self {
            start_value: 0,
            current_value: 0,
            digit: 0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl Ticker {

    /// Develops a new ticker using a passed value for its start_value.
    ///
    /// # TICKING LOOPS AT I8::MAX
    /// Tickers don't stop ticking.  Once the next tick addition hits i8::MAX it will zero out current_value using to_zero().
    /// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
    /// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
    /// **If you're okay with values from -128 to 127 for your timers, then feel free to go ham with Tickers.**  Otherwise,
    /// I recommend the Chronolog structure.
    pub fn new(starting_value: i8) -> Self {
        Self {
            start_value:    starting_value,
            current_value:  starting_value,
            digit:          (starting_value as i16).abs() as i8 % 10,                   // Have to cast a bit extra due to the possibility that start value is i8::MIN (-128 flipping to 128 is out of i8 range).
            timer:          Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }

    /// Returns the current_value of a Ticker.
    pub fn get_current_value(&self) -> i8 {
        self.current_value
    }

    /// Returns the start_value of a Ticker.
    ///
    /// It's important to note that start_value can change through set_start_value(), so don't
    /// treat it as a consistent value.
    pub fn get_start_value(&self) -> i8 {
        self.start_value
    }

    /// Returns the total amount of time that has passed from the start_value.  This can technically return
    /// misleading values depending on how start_value and current_value are manipulated.
    ///
    /// If 5 seconds is added to the current_value, then 5 seconds also gets increased to the elapsed time.
    /// If 5 seconds is reduced from the current_value, then 5 seconds is reduced from the elapsed time.
    ///
    /// If start_value is set to a different value, then the elapsed time gets completely changed.
    ///
    /// The elapsed value is always positive for sanity reasons.  Use is_above_start and is_below_start
    /// to determine where your current_value sits against start_value.
    pub fn get_elapsed_value(&self) -> i8 {
        ((self.current_value as i16) - (self.start_value as i16)).abs() as i8       // Have to cast a bit extra due to the possibility that start value is i8::MIN (-128 flipping to 128 is out of i8 range).
    }

    /// Returns the digit in the ones-place of the current_value.
    pub fn get_digit(&self) -> i8 {
        self.digit
    }

    /// Will return the Bevy timer being used in the Ticker.
    ///
    /// To my knowledge, this method is for the most part useless since Tickers are only assigned
    /// to repeating Bevy timers that use from_second with a value of 1.0.  BUT, in the case that I'm
    /// wrong, this method is around for anybody that needs to get the Timer inside a Ticker.
    pub fn get_timer(&self) -> &Timer {
        &self.timer
    }

    /// Allows manipulation of the current_value.
    ///
    /// Both start_value and current_value have setters to allow for time manipulation shenanigans.  If an
    /// event were to occur and someone wanted to drastically alter how time worked then they can use the
    /// setters to make some interesting mechanics.
    pub fn set_current_value(&mut self, value: i8) {
        self.current_value = value;
    }

    /// Allows manipulation of the start_value.
    ///
    /// Both start_value and current_value have setters to allow for time manipulation shenanigans.  If an
    /// event were to occur and someone wanted to drastically alter how time worked then they can use the
    /// setters to make some interesting mechanics.
    pub fn set_start_value(&mut self, value: i8) {
        self.start_value = value;
    }

    /// Pauses a timer within the ticker.
    pub fn pause(&mut self) {
        self.timer.pause();
    }

    /// Unpauses a timer within a ticker.
    pub fn unpause(&mut self) {
        self.timer.unpause();
    }

    /// Will set the current_value to be equal to the start_value and the digit field of the Ticker
    /// will be changed according to the new ones-place value that is seen after current_value's reset.
    ///
    /// Digit is always to reflect current_value's ones-place.
    pub fn reset(&mut self) {
        self.current_value = self.start_value;
        self.digit = (self.current_value as i16).abs() as i8 % 10;      // Have to cast a bit extra due to the possibility that start value is i8::MIN (-128 flipping to 128 is out of i8 range).
    }

    /// Adds to the start_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping.
    pub fn add_to_start(&mut self, value: i8) {
        self.start_value = self.start_value.saturating_add(value);
    }

    /// Adds to the current_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping.
    pub fn add_to_current(&mut self, value: i8) {
        self.current_value = self.current_value.saturating_add(value);
    }

    /// Returns true if the current_value of the Ticker is below its start_value, false otherwise.
    pub fn current_is_below_start(&self) -> bool {
        self.current_value < self.start_value
    }

    /// Returns true if the current_value of the Ticker is above its start_value, false otherwise.
    pub fn current_is_above_start(&self) -> bool {
        self.current_value > self.start_value
    }

    /// Returns true if the current_value and the start_value are equal to one another, false otherwise.
    ///
    /// When relying solely on frames, I think this would be rather difficult to trigger.  However,
    /// using the reset method and setters may allow for this to return true often depending on
    /// how said methods are used.
    pub fn current_is_equal_to_start(&self) -> bool {
        self.current_value == self.start_value
    }

    /// Will make the current_value and digit be set to zero.
    ///
    /// Zero is a special number which is why it gets its own method.  Never let anybody tell you that
    /// zero isn't special - it's the almighty equalizer, destroyer, and splitter.
    pub fn to_zero(&mut self) {
        self.current_value = 0;
        self.digit = 0;
    }

    /// Sets current_value to its minimum value (will alter the digit field to reflect this change).
    pub fn to_min(&mut self) {
        self.current_value = i8::MIN;
        self.digit = 8;
    }

    /// Sets current_value to its maximum value (will alter the digit field to reflect this change).
    pub fn to_max(&mut self) {
        self.current_value = i8::MAX;
        self.digit = 7;
    }

    /// Used to advance a ticker.  Takes in a time.delta() call off the time resource (Res<Time>) that Bevy provides.
    ///
    /// If you're making a custom ticking system and have stripped out the ticking systems provided
    /// in the systems of this plugin, then please note that you must run this each frame for time to move normally.
    ///
    /// # TICKING LOOPS AT I8::MAX
    /// Tickers don't stop ticking.  Once the next tick addition hits i8::MAX it will zero out current_value using to_zero().
    /// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
    /// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
    /// **If you're okay with values from -128 to 127 for your timers, then feel free to go ham with Tickers.**  Otherwise,
    /// I recommend the Chronolog structure.
    pub fn tick(&mut self, delta: std::time::Duration) {

        // Advance timer by the difference in time between frames.
        // This .tick is Bevy's tick method for their timers, this isn't a recursive action.
        self.timer.tick(delta);

        // Handling frame spiking.
        let ticks: u32 = self.timer.times_finished_this_tick();
        if ticks > 0 {

            let new_ticks: i8 = ticks as i8;

            // This condition is effectively resetting the current_value and digit to zero if the i8
            // max value gets hit from tick addition.  This allows tickers to tick forever.
            if self.current_value.saturating_add(new_ticks) == i8::MAX {
                self.to_zero();
            }
            else {
                self.current_value = self.current_value.saturating_add(new_ticks);
                self.digit = (self.digit + new_ticks).abs() % 10;
            }
        }
    }
}

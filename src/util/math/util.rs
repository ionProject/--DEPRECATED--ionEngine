/*===============================================================================================*/
// Copyright 2016 Kyle Finlay
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
/*===============================================================================================*/

/*===============================================================================================*/
/*------UTIL STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The Math utility struct
///
/// It contains various utility functions such as: Clamp, Lerp, Min, Max, etc.
#[derive (Copy, Clone)]
pub struct Util;

/*===============================================================================================*/
/*------UTIL PUBLIC STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

impl Util {

    /// Clamps a value between two numbers.
    ///
    /// # Arguments
    /// * `value` - The value to clamp
    /// * `min` - The minimum the value is allowed to be
    /// * `max` - The maximum the value is allowed to be
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Util;
    ///
    /// let val = 23.0;
    /// let min = 5.0;
    /// let max = 19.0;
    ///
    /// println! ("Clamp = {}", Util::clamp (val, min, max));
    /// ```
    /// ```c
    /// Output : Clamp = 19.0
    pub fn clamp (value: f32, min: f32, max: f32) -> f32 {

        if value < min {
            return min;
        }

        else if value > max {
            return max;
        }

        value
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Converts a number from degrees to radians.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Util;
    ///
    /// let deg = 95.0;
    /// println! ("Value as radians = {}", Util::deg2rad (deg));
    /// ```
    /// ```c
    /// Output : Value as radians = 1.658035
    pub fn deg2rad (value: f32) -> f32 {

        value * 0.017453
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Linearly interpolates between two numbers.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Util;
    ///
    /// let start = 9.0;
    /// let end = 22.0;
    /// let percent = 0.5;
    ///
    /// println! ("{}", Util::lerp (start, end, percent));
    /// ```
    /// ```c
    /// Output : 15.5
    pub fn lerp (start: f32, end: f32, percentage: f32) -> f32 {

        start + (end - start) * Util::clamp (percentage, 0.0, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Linearly interpolates between two numbers without clamping.
    pub fn lerp_unclamped (start: f32, end: f32, percentage: f32) -> f32 {

        start + (end - start) * percentage
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the largest of two numbers
    ///
    /// # Arguments
    /// * `lhs` - The first value to compare, or left hand side value
    /// * `rhs` - The second value to compare, or right hand side value
    pub fn max (lhs: f32, rhs: f32) -> f32 {

        if lhs > rhs {
            return lhs
        }

        rhs
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the smallest of two numbers
    ///
    /// # Arguments
    /// * `lhs` - The first value to compare, or left hand side value
    /// * `rhs` - The second value to compare, or right hand side value
    pub fn min (lhs: f32, rhs: f32) -> f32 {

        if lhs < rhs {
            return lhs;
        }

        rhs
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Converts a number from radians to degrees
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Util;
    ///
    /// let rad = 2.5;
    /// println! ("Value as degrees = {}", Util::rad2deg (rad));
    /// ```
    /// ```c
    /// Output : Value as degrees = 143.2394475
    pub fn rad2deg (value: f32) -> f32 {

        value * 57.295779
    }
}

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

// Crate imports
extern crate num_traits;

// Module imports
use self::num_traits::{Float, NumCast};

use ::angle::Deg;

use std::convert::From;

/*===============================================================================================*/
/*------RAD STRUCT-------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores a value in Radians.
#[derive (Copy, Clone, Debug, Default)]
pub struct Rad<V> where V: Copy + Float + NumCast {

    // Public
    /// The value of the radian.
    pub value: V,
}

/*===============================================================================================*/
/*------RAD TRAIT IMPLEMENTATIONS----------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> From<Deg<V>> for Rad<V> where V: Copy + Float + NumCast {

    fn from (deg: Deg<V>) -> Rad<V> {

        Rad::new (deg.value * V::from (0.017453).unwrap ())
    }
}

/*===============================================================================================*/
/*------RAD PUBLIC METHODS-----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Rad<V> where V: Copy + Float + NumCast {

    /// Returns a new `Rad` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::angle::{Deg, Rad};
    /// let rad = Rad::<f64>::new (2.641);
    /// ```
    pub fn new (value: V) -> Rad<V> {

        Rad {value: value}
    }
}


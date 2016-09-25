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
use self::num_traits::{Float, Num, NumCast};

use std::ops::Sub;

/*===============================================================================================*/
/*------VEC2 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec2 struct.
///
/// It is used mainly for 2D related mathematics such as texture coordinates,
/// UV coordinates, etc. <br>
/// It can accept any number as a value.
#[derive (Debug)]
pub struct Vec2<V> where V: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: V,
    /// The vector y-coordinate.
    pub y: V,
}

// Predefined Vec2 types
pub type Vec2f = Vec2<f32>;
pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;

/*===============================================================================================*/
/*------VEC2<V> TRAIT IMPLEMENTATIONS------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Default for Vec2<V> where V: Copy + Num + NumCast {

    fn default () -> Vec2<V> {
        Vec2::<V>::zero ()
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, V> Sub<&'b Vec2<V>> for &'a Vec2<V>
    where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: &'b Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Sub<V> for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x - rhs,
                   self.y - rhs)
    }
}

/*===============================================================================================*/
/*------VEC2<V> PUBLIC METHODS-------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Vec2<V> where V: Copy + Num + NumCast {

    /// Returns the dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec01 = Vec2::new (1, 3);
    /// let vec02 = Vec2::new (4, 9);
    ///
    /// let dot_product = vec01.dot (&vec02);
    /// ```
    pub fn dot (&self, rhs: &Vec2<V>) -> V {

        (self.x * rhs.x) +
        (self.y * rhs.y)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a new `Vec2<V>` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::new (3, 7);
    /// ```
    pub fn new (x: V, y: V) -> Vec2<V> {

        Vec2 {x: x,
              y: y}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of zero.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::zero ();
    /// ```
    pub fn zero () -> Vec2<V> {

        Vec2 {x: V::zero (),
              y: V::zero ()}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, 1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::up ();
    /// ```
    pub fn up () -> Vec2<V> {

        Vec2 {x: V::zero (),
              y: V::one  ()}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, -1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::down ();
    /// ```
    pub fn down () -> Vec2<V> {

        Vec2 {x: V::zero (),
              y: V::from (-1).unwrap ()}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, 1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::right ();
    /// ```
    pub fn right () -> Vec2<V> {

        Vec2 {x: V::one  (),
              y: V::zero ()}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<V>` with a value of (0, -1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::left ();
    /// ```
    pub fn left () -> Vec2<V> {

        Vec2 {x: V::from (-1).unwrap (),
              y: V::zero ()}
    }
}

/*===============================================================================================*/
/*------VEC2<Float> PUBLIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Vec2<V> where V: Float {

    /// Returns the distance between two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec01 = Vec2::new (1.0, 3.0);
    /// let vec02 = Vec2::new (4.0, 9.0);
    ///
    /// let distance = vec01.distance (&vec02);
    /// println! ("Distance: {}", distance);
    /// ```
    pub fn distance (&self, rhs: &Vec2<V>) -> V {

        (self - rhs).length ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::new (1.0, 3.0);
    /// let vec_length = vec.length ();
    /// ```
    pub fn length (&self) -> V {

        (self.x * self.x +
         self.y * self.y).sqrt ()
    }
}

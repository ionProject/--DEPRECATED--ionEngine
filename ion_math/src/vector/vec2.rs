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

use ::vector::Vec3;

use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};
use std::convert::From;

/*===============================================================================================*/
/*------VEC2 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec2 struct.
///
/// It is used mainly for 2D related mathematics such as texture coordinates,
/// UV coordinates, etc. <br>
/// It can accept any number as a value.
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2<V> where V: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: V,
    /// The vector y-coordinate.
    pub y: V,
}

// Predefined Vec2 types
/// `Vec2<f32>`
pub type Vec2f = Vec2<f32>;
/// `Vec2<i32>`
pub type Vec2i = Vec2<i32>;
/// `Vec2<u32>`
pub type Vec2u = Vec2<u32>;

/*===============================================================================================*/
/*------VEC2 TRAIT IMPLEMENTATIONS---------------------------------------------------------------*/
/*===============================================================================================*/

impl<V, U> From<Vec3<U>> for Vec2<V>
    where V: Copy + Num + NumCast,
          U: Copy + Num + NumCast {

    fn from (value: Vec3<U>) -> Vec2<V> {

        Vec2::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap ())
    }
}

/*===============================================================================================*/
/*------VEC2 OPERATORS---------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Add for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Add<V> for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn add (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x + rhs,
                   self.y + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign for Vec2<V> where V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> AddAssign<V> for Vec2<V> where V: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: V) {

        self.x = self.x + rhs;
        self.y = self.y + rhs;
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

impl<V> Sub<V> for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn sub (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x - rhs,
                   self.y - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign for Vec2<V> where V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> SubAssign<V> for Vec2<V> where V: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: V) {

        self.x = self.x - rhs;
        self.y = self.y - rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Mul<V> for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn mul (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x * rhs,
                   self.y * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign for Vec2<V> where V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> MulAssign<V> for Vec2<V> where V: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: V) {

        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: Vec2<V>) -> Vec2<V> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Div<V> for Vec2<V> where V: Copy + Num + NumCast {

    type Output = Vec2<V>;

    fn div (self, rhs: V) -> Vec2<V> {

        Vec2::new (self.x / rhs,
                   self.y / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign for Vec2<V> where V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: Vec2<V>) {

        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> DivAssign<V> for Vec2<V> where V: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: V) {

        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> Index<u8> for Vec2<V> where V: Copy + Num + NumCast {

    type Output = V;

    fn index (&self, index: u8) -> &V {

        match index {

            0 => &self.x,
            1 => &self.y,
            _ => unreachable! ("Index out of range for Vec2")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<V> IndexMut<u8> for Vec2<V> where V: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut V {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable! ("Index out of range for Vec2")
        }
    }
}

/*===============================================================================================*/
/*------VEC2 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<V> Vec2<V> where V: Copy + Num + NumCast {

    /// Converts self into an instance of `Vec2<C>`.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec01: Vec2<f32> = Vec2::new (4.3, 9.8);
    /// let vec02: Vec2<i32> = vec01.into ();
    /// ```
    pub fn into<C> (self) -> Vec2<C> where C: Copy + Num + NumCast {

        Vec2::new (C::from (self.x).unwrap (),
                   C::from (self.y).unwrap ())
    }

/*-----------------------------------------------------------------------------------------------*/

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

    /// Returns a new instance of `Vec2<V>` from an instance of `Vec2<C>`.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec01: Vec2<f32> = Vec2::new  (3.2, 9.8);
    /// let vec02: Vec2<i32> = Vec2::from (vec01);
    /// ```
    pub fn from<C> (value: Vec2<C>) -> Vec2<V> where C: Copy + Num + NumCast {

        Vec2::new (V::from (value.x).unwrap (),
                   V::from (value.y).unwrap ())
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

/*-----------------------------------------------------------------------------------------------*/

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
    /// ```
    pub fn distance (&self, rhs: &Vec2<V>) -> V {

        (*self - *rhs).length ()
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

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a normalized vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::new (3.0, 9.0);
    /// let vec_normalized = vec.normalize ();
    /// ```
    pub fn normalize (&self) -> Vec2<V> {

        let length = self.length ();

        if length != V::zero () {

            return Vec2::<V>::new (self.x / length,
                                   self.y / length);
        }

        Vec2::<V>::zero ()
    }
}

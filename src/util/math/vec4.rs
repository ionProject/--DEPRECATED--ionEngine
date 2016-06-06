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

use ::util::math::Util;

use std::ops::*;
use std::cmp::PartialEq;

/*===============================================================================================*/
/*------VEC4 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The Vec4 struct
///
/// This struct represents 4D vectors and points.
/// It is used for things such as mesh tangets, and shader parameters.
/// 32-bit floats are used for the values.
#[derive (Copy, Clone, Default)]
pub struct Vec4 {

    // Public
    /// X-axis coordinate
    pub x: f32,
    /// Y-axis coordinate
    pub y: f32,
    /// Z-axis coordinate
    pub z: f32,
    /// W-axis coordinate
    pub w: f32
}

/*===============================================================================================*/
/*------VEC4 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl Vec4 {

    /// Formats the vector as a string.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Vec4;
    ///
    /// let vec = Vec4 {x : 10.0, y : 5.0, z : 1.0, w : 0.0};
    /// println! ("Vector = {}", vec.to_string ());
    /// ```
    /// ```c
    /// Output : Vector = 10.0, 5.0, 1.0, 0.0
    pub fn to_string (&self) -> String {

        format! ("{}, {}, {}, {}", self.x, self.y, self.z, self.w)
    }

/*===============================================================================================*/
/*------VEC4 PUBLIC STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

    /// Creates a vector with a default value of zero.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Vec4;
    ///
    /// let vec = Vec4::new ();
    pub fn new () -> Vec4 {

        Vec4 {x: 0.0,
              y: 0.0,
              z: 0.0,
              w: 0.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the dot product of two vectors.
    pub fn dot (lhs: &Vec4, rhs: &Vec4) -> f32 {

        (lhs.x * rhs.x) +
        (lhs.y * rhs.y) +
        (lhs.z * rhs.z) +
        (lhs.w * rhs.w)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the distance between two vectors
    pub fn distance (start: &Vec4, end: &Vec4) -> f32 {

        Vec4::length (& (*start - *end))
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector
    pub fn length (vector: &Vec4) -> f32 {

        (vector.x * vector.x +
         vector.y * vector.y +
         vector.z * vector.z).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Linearly interpolates between two vectors.
    pub fn lerp (start: &Vec4, end: &Vec4, percentage: f32) -> Vec4 {

        Vec4 {x: Util::lerp (start.x, end.x, percentage),
              y: Util::lerp (start.y, end.y, percentage),
              z: Util::lerp (start.z, end.z, percentage),
              w: Util::lerp (start.w, end.w, percentage)}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Linearly interpolates between two vectors without clamping
    pub fn lerp_unclamped (start: &Vec4, end: &Vec4, percentage: f32) -> Vec4 {

        Vec4 {x: Util::lerp_unclamped (start.x, end.x, percentage),
              y: Util::lerp_unclamped (start.y, end.y, percentage),
              z: Util::lerp_unclamped (start.z, end.z, percentage),
              w: Util::lerp_unclamped (start.z, end.z, percentage)}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Get a normalized vector.
    pub fn normalize (vector: &Vec4) -> Vec4 {

        let length = Vec4::length (vector);

        if length != 0.0 {

            return Vec4 {x: vector.x / length,
                         y: vector.y / length,
                         z: vector.z / length,
                         w: vector.w / length}
        }

        Vec4::new ()
    }
}

/*===============================================================================================*/
/*------VEC4 OPERATOR OVERLOADS------------------------------------------------------------------*/
/*===============================================================================================*/

impl Add for Vec4 {

    type Output = Vec4;

    // Addition operator (vector)
    fn add (self, rhs: Vec4) -> Vec4 {

        Vec4 {x: self.x + rhs.x,
              y: self.y + rhs.y,
              z: self.z + rhs.z,
              w: self.w + rhs.w}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Add <f32> for Vec4 {

    type Output = Vec4;

    // Addition operator (f32)
    fn add (self, rhs: f32) -> Vec4 {

        Vec4 {x: self.x + rhs,
              y: self.y + rhs,
              z: self.z + rhs,
              w: self.w + rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign for Vec4 {

    // Addition assignment operator (vector)
    fn add_assign (&mut self, rhs: Vec4) {

        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign <f32> for Vec4 {

    // Addition assignment operator (f32)
    fn add_assign (&mut self, rhs: f32) {

        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub for Vec4 {

    type Output = Vec4;

    // Subtraction operator (vector)
    fn sub (self, rhs: Vec4) -> Vec4 {

        Vec4 {x: self.x - rhs.x,
              y: self.y - rhs.y,
              z: self.z - rhs.z,
              w: self.w - rhs.w}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub <f32> for Vec4 {

    type Output = Vec4;

    // Subtraction operator (f32)
    fn sub (self, rhs: f32) -> Vec4 {

        Vec4 {x: self.x - rhs,
              y: self.y - rhs,
              z: self.z - rhs,
              w: self.w - rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign for Vec4 {

    // Subtraction assignment operator (vector)
    fn sub_assign (&mut self, rhs: Vec4) {

        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign <f32> for Vec4 {

    // Subtraction assignment operator (f32)
    fn sub_assign (&mut self, rhs: f32) {

        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Neg for Vec4 {

    type Output = Vec4;

    // Unary minus operator
    fn neg (self) -> Vec4 {

        Vec4 {x: -self.x,
              y: -self.y,
              z: -self.z,
              w: -self.w}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul for Vec4 {

    type Output = Vec4;

    // Multiplication operator (vector)
    fn mul (self, rhs: Vec4) -> Vec4 {

        Vec4 {x: self.x * rhs.x,
              y: self.y * rhs.y,
              z: self.z * rhs.z,
              w: self.w * rhs.w}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul <f32> for Vec4 {

    type Output = Vec4;

    // Multiplication operator (f32)
    fn mul (self, rhs: f32) -> Vec4 {

        Vec4 {x: self.x * rhs,
              y: self.y * rhs,
              z: self.z * rhs,
              w: self.w * rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl MulAssign for Vec4 {

    // Multiplication assignment operator (vector)
    fn mul_assign (&mut self, rhs: Vec4) {

        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl MulAssign <f32> for Vec4 {

    // Multiplication assignment operator (f32)
    fn mul_assign (&mut self, rhs: f32) {

        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Div for Vec4 {

    type Output = Vec4;

    // Division operator (vector)
    fn div (self, rhs: Vec4) -> Vec4 {

        Vec4 {x: self.x / rhs.x,
              y: self.y / rhs.y,
              z: self.z / rhs.z,
              w: self.w / rhs.w}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Div <f32> for Vec4 {

    type Output = Vec4;

    // Division operator (f32)
    fn div (self, rhs: f32) -> Vec4 {

        Vec4 {x: self.x / rhs,
              y: self.y / rhs,
              z: self.z / rhs,
              w: self.w / rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl DivAssign for Vec4 {

    // Division assignment operator (vector)
    fn div_assign (&mut self, rhs: Vec4) {

        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl DivAssign <f32> for Vec4 {

    // Division assignment operator (f32)
    fn div_assign (&mut self, rhs: f32) {

        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl PartialEq for Vec4 {

    // Equal to operator
    fn eq (&self, rhs: &Vec4) -> bool {

        self.x == rhs.x &&
        self.y == rhs.y &&
        self.z == rhs.z &&
        self.w == rhs.w
    }

/*-----------------------------------------------------------------------------------------------*/

    // Not equal to operator
    fn ne (&self, rhs: &Vec4) -> bool {

        self.x != rhs.x ||
        self.y != rhs.y ||
        self.z != rhs.z ||
        self.w != rhs.w
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Index <u8> for Vec4 {

    type Output = f32;

    // Index operator (immutable)
    fn index (&self, index: u8) -> &f32 {

        match index {

            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => unreachable! ("Index out of range for Vec4")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl IndexMut <u8> for Vec4 {

    // Index operator (mutable)
    fn index_mut (&mut self, index: u8) -> &mut f32 {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => unreachable! ("Index out of range for Vec4")
        }
    }
}

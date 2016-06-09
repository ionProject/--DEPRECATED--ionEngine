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
/*------VEC3 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The Vec3 struct
///
/// This is the most used out of the three vector structs.
/// It is used for representing positions in 3D space (vertices, normals, positions, etc).
/// 32-bit floats are used for the values.
#[derive (Copy, Clone, Default)]
pub struct Vec3 {

    // Public
    /// X-axis coordinate
    pub x: f32,
    /// Y-axis coordinate
    pub y: f32,
    /// Z-axis coordinate
    pub z: f32
}

/*===============================================================================================*/
/*------VEC3 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl Vec3 {

    /// Formats the vector as a string.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::Vec3;
    /// #
    /// let vec = Vec3 {x : 10.0, y : 5.0, z : 1.0};
    /// println! ("Vector = {}", vec.to_string ());
    /// ```
    /// ```c
    /// Output : Vector = 10.0, 5.0, 1.0
    pub fn to_string (&self) -> String {

        format! ("{}, {}, {}", self.x, self.y, self.z)
    }

/*===============================================================================================*/
/*------VEC3 PUBLIC STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

    /// Creates a vector with a default value of zero.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::Vec3;
    /// #
    /// let vec = Vec3::new ();
    pub fn new () -> Vec3 {

        Vec3 {x: 0.0,
              y: 0.0,
              z: 0.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a vector with a value of (0, 1, 0).
    pub fn up () -> Vec3 {

        Vec3 {x: 0.0,
              y: 1.0,
              z: 0.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a vector with a value of (0, -1, 0).
    pub fn down () -> Vec3 {

        Vec3 {x: 0.0,
              y: -1.0,
              z: 0.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a vector with a value of (-1, 0, 0).
    pub fn left () -> Vec3 {

        Vec3 {x: -1.0,
              y: 0.0,
              z: 0.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a vector with a value of (1, 0, 0)
    pub fn right () -> Vec3 {

        Vec3 {x: 1.0,
              y: 0.0,
              z: 0.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a vector with a value of (0, 0, 1)
    pub fn forward () -> Vec3 {

        Vec3 {x: 0.0,
              y: 0.0,
              z: 1.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Cretaes a vector with a value of (0, 0, -1)
    pub fn back () -> Vec3 {

        Vec3 {x: 0.0,
              y: 0.0,
              z: -1.0}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Retuns the cross product of two vectors.
    pub fn cross (lhs: &Vec3, rhs: &Vec3) -> Vec3 {

        Vec3 {x: lhs.y * rhs.z - rhs.y * lhs.z,
              y: lhs.z * rhs.x - rhs.x * lhs.x,
              z: lhs.x * rhs.y - rhs.x * lhs.y}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the dot product of two vectors.
    pub fn dot (lhs: &Vec3, rhs: &Vec3) -> f32 {

        (lhs.x * rhs.x) +
        (lhs.y * rhs.y) +
        (lhs.z * rhs.z)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the distance between two vectors
    pub fn distance (start: &Vec3, end: &Vec3) -> f32 {

        Vec3::length (& (*start - *end))
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector
    pub fn length (vector: &Vec3) -> f32 {

        (vector.x * vector.x +
         vector.y * vector.y +
         vector.z * vector.z).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Linearly interpolates between two vectors.
    pub fn lerp (start: &Vec3, end: &Vec3, percentage: f32) -> Vec3 {

        Vec3 {x: Util::lerp (start.x, end.x, percentage),
              y: Util::lerp (start.y, end.y, percentage),
              z: Util::lerp (start.z, end.z, percentage)}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Linearly interpolates between two vectors without clamping
    pub fn lerp_unclamped (start: &Vec3, end: &Vec3, percentage: f32) -> Vec3 {

        Vec3 {x: Util::lerp_unclamped (start.x, end.x, percentage),
              y: Util::lerp_unclamped (start.y, end.y, percentage),
              z: Util::lerp_unclamped (start.z, end.z, percentage)}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Get a normalized vector.
    pub fn normalize (vector: &Vec3) -> Vec3 {

        let length = Vec3::length (vector);

        if length != 0.0 {

            return Vec3 {x: vector.x / length,
                         y: vector.y / length,
                         z: vector.z / length}
        }

        Vec3::new ()
    }
}

/*===============================================================================================*/
/*------VEC3 OPERATOR OVERLOADS------------------------------------------------------------------*/
/*===============================================================================================*/

impl Add for Vec3 {

    type Output = Vec3;

    // Addition operator (vector)
    fn add (self, rhs: Vec3) -> Vec3 {

        Vec3 {x: self.x + rhs.x,
              y: self.y + rhs.y,
              z: self.z + rhs.z}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Add <f32> for Vec3 {

    type Output = Vec3;

    // Addition operator (f32)
    fn add (self, rhs: f32) -> Vec3 {

        Vec3 {x: self.x + rhs,
              y: self.y + rhs,
              z: self.z + rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign for Vec3 {

    // Addition assignment operator (vector)
    fn add_assign (&mut self, rhs: Vec3) {

        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign <f32> for Vec3 {

    // Addition assignment operator (f32)
    fn add_assign (&mut self, rhs: f32) {

        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub for Vec3 {

    type Output = Vec3;

    // Subtraction operator (vector)
    fn sub (self, rhs: Vec3) -> Vec3 {

        Vec3 {x: self.x - rhs.x,
              y: self.y - rhs.y,
              z: self.z - rhs.z}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub <f32> for Vec3 {

    type Output = Vec3;

    // Subtraction operator (f32)
    fn sub (self, rhs: f32) -> Vec3 {

        Vec3 {x: self.x - rhs,
              y: self.y - rhs,
              z: self.z - rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign for Vec3 {

    // Subtraction assignment operator (vector)
    fn sub_assign (&mut self, rhs: Vec3) {

        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign <f32> for Vec3 {

    // Subtraction assignment operator (f32)
    fn sub_assign (&mut self, rhs: f32) {

        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Neg for Vec3 {

    type Output = Vec3;

    // Unary minus operator
    fn neg (self) -> Vec3 {

        Vec3 {x: -self.x,
              y: -self.y,
              z: -self.z}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul for Vec3 {

    type Output = Vec3;

    // Multiplication operator (vector)
    fn mul (self, rhs: Vec3) -> Vec3 {

        Vec3 {x: self.x * rhs.x,
              y: self.y * rhs.y,
              z: self.z * rhs.z}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul <f32> for Vec3 {

    type Output = Vec3;

    // Multiplication operator (f32)
    fn mul (self, rhs: f32) -> Vec3 {

        Vec3 {x: self.x * rhs,
              y: self.y * rhs,
              z: self.z * rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl MulAssign for Vec3 {

    // Multiplication assignment operator (vector)
    fn mul_assign (&mut self, rhs: Vec3) {

        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl MulAssign <f32> for Vec3 {

    // Multiplication assignment operator (f32)
    fn mul_assign (&mut self, rhs: f32) {

        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Div for Vec3 {

    type Output = Vec3;

    // Division operator (vector)
    fn div (self, rhs: Vec3) -> Vec3 {

        Vec3 {x: self.x / rhs.x,
              y: self.y / rhs.y,
              z: self.z / rhs.z}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Div <f32> for Vec3 {

    type Output = Vec3;

    // Division operator (f32)
    fn div (self, rhs: f32) -> Vec3 {

        Vec3 {x: self.x / rhs,
              y: self.y / rhs,
              z: self.z / rhs}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl DivAssign for Vec3 {

    // Division assignment operator (vector)
    fn div_assign (&mut self, rhs: Vec3) {

        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl DivAssign <f32> for Vec3 {

    // Division assignment operator (f32)
    fn div_assign (&mut self, rhs: f32) {

        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl PartialEq for Vec3 {

    // Equal to operator
    fn eq (&self, rhs: &Vec3) -> bool {

        self.x == rhs.x &&
        self.y == rhs.y &&
        self.z == rhs.z
    }

/*-----------------------------------------------------------------------------------------------*/

    // Not equal to operator
    fn ne (&self, rhs: &Vec3) -> bool {

        self.x != rhs.x ||
        self.y != rhs.y ||
        self.z != rhs.z
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Index <u8> for Vec3 {

    type Output = f32;

    // Index operator (immutable)
    fn index (&self, index: u8) -> &f32 {

        match index {

            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable! ("Index out of range for Vec3")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl IndexMut <u8> for Vec3 {

    // Index operator (mutable)
    fn index_mut (&mut self, index: u8) -> &mut f32 {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable! ("Index out of range for Vec3")
        }
    }
}

/*================================================================================================*/
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
/*================================================================================================*/

use ::util::math::{Util, Vec2, Vec3};

use std::ops::*;
use std::cmp::PartialEq;

/*================================================================================================*/
/*------MAT3 STRUCT-------------------------------------------------------------------------------*/
/*================================================================================================*/

/// 3x3 Matrix.
///
/// This struct represents a 3x3 matrix. It is normally used for 2D graphics transformations,
/// such as translation, rotation, and scaling.
#[derive (Copy, Clone, Default)]
pub struct Mat3 {

    // Private
    _value: [Vec3; 3]
}

/*================================================================================================*/
/*------MAT3 PUBLIC METHODS-----------------------------------------------------------------------*/
/*================================================================================================*/

impl Mat3 {

    /// Formats the matrix as a string.
    ///
    /// # Return value
    /// The matrix formatted as a string.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Mat3;
    ///
    /// let mat = Mat3::identity ();
    /// println! ("{}", mat.to_string ());
    /// ```
    /// ```c
    /// Output : [1, 0, 0]
    ///          [0, 1, 0]
    ///          [0, 0, 1]
    pub fn to_string (&self) -> String {

        format! ("[{}]\n[{}]\n[{}]", self[0].to_string (),
                                     self[1].to_string (),
                                     self[2].to_string ())
    }

/*================================================================================================*/
/*------MAT3 PUBLIC STATIC METHODS----------------------------------------------------------------*/
/*================================================================================================*/

    /// Creates a matrix with default values.
    ///
    /// # Return value
    /// A new matrix containing all zeros.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Mat3;
    ///
    /// let mat = Mat3::new (); // Returns a matrix with all zeros
    pub fn new () -> Mat3 {

        Mat3 {_value: [Vec3::new (); 3]}
    }

    /// Returns a new identity matrix.
    ///
    /// # Return value
    /// A new identity matrix instance.
    pub fn identity () -> Mat3 {

        Mat3 {_value: [Vec3 {x: 1.0, y: 0.0, z: 0.0},
                       Vec3 {x: 0.0, y: 1.0, z: 0.0},
                       Vec3 {x: 0.0, y: 0.0, z: 1.0}]}
    }

    /// Returns the determinant of a matrix.
    /// This is used as part of the inverse matrix calculation.
    ///
    /// # Arguments
    /// * `m` - Reference to the matrix you wish to get the determinant from.
    ///
    /// # Return value
    /// The determinant of the matrix.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Mat3;
    ///
    /// let mat = Mat3::identity ();
    /// println! ("Determinant = {}", Mat3::determinant (&mat));
    /// ```
    /// ```c
    /// Output : Determinant = 1.0
    pub fn determinant (m: &Mat3) -> f32 {

        m[0][0] * m[1][1] * m[2][2] +
        m[0][1] * m[1][2] * m[2][0] +
        m[0][2] * m[1][0] * m[2][1] -
        m[0][0] * m[1][2] * m[2][1] -
        m[0][1] * m[1][0] * m[2][2] -
        m[0][2] * m[1][1] * m[2][0]
    }

    /// Returns the inverse of a matrix.
    ///
    /// When multiplied by the original matrix, the end result will be an identity matrix.
    ///
    /// # Arguments
    /// * `m` - The reference to the matrix you wish to get the inverse of.
    ///
    /// # Return value
    /// A new matrix which is the inverse of m.
    pub fn inverse (m: &Mat3) -> Mat3 {

        let det = 1.0 / Mat3::determinant (m);

        Mat3 {_value: [Vec3 {x: (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * det,
                             y: (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * det,
                             z: (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * det},

                       Vec3 {x: (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * det,
                             y: (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * det,
                             z: (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * det},

                       Vec3 {x: (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * det,
                             y: (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * det,
                             z: (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * det}]}
    }

    /// Returns a 2D translation matrix.
    ///
    /// It represents a transformation in 2D space.
    /// When combined with a rotation and scale matrix, it creates a model matrix.
    ///
    /// # Arguments
    /// * `position` - A 'Vec2' representing the position.
    ///
    /// # Return value
    /// A new translation matrix.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::{Mat3, Vec2};
    ///
    /// let pos = Vec2 {x : 10.0, y : 43.0};
    /// let mat = Mat3::translate (&pos);
    pub fn translate (position: &Vec2) -> Mat3 {

        Mat3 {_value: [Vec3 {x: 1.0, y: 0.0, z: position.x},
                       Vec3 {x: 0.0, y: 1.0, z: position.y},
                       Vec3 {x: 0.0, y: 0.0, z: 1.0}]}
    }

    /// Returns a 2D rotation matrix.
    ///
    /// It represents a rotation in 2D space.
    /// When combined with a translation and scale matrix, it creates a model matrix.
    ///
    /// It takes in a single f32 argument, which is the rotation in degrees.
    ///
    /// # Arguments
    /// * `rotation` - The desired rotation in degrees.
    ///
    /// # Return value
    /// A new rotation matrix.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Mat3;
    ///
    /// let rot = 45.0;
    /// let mat = Mat3::rotate (rot);
    pub fn rotate (rotation: f32) -> Mat3 {

        // Convert rotation to radians
        let rad_rotation = Util::deg2rad (rotation);

        let z_sin = rad_rotation.sin ();
        let z_cos = rad_rotation.cos ();

        // Transform z-axis
        Mat3 {_value: [Vec3 {x: z_cos, y: -z_sin, z: 0.0},
                       Vec3 {x: z_sin, y: z_cos,  z: 0.0},
                       Vec3 {x: 0.0,   y: 0.0,    z: 1.0}]}
    }

    /// Returns a 2D scale matrix.
    ///
    /// It represents a scale in 2D space.
    /// When combined with a rotation and rotation matrix, it creates a model matrix.
    ///
    /// # Arguments
    /// * `scale` - A Vec2 representing the desired scale.
    ///
    /// # Return value
    /// A new scale matrix.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::{Mat3, Vec2};
    ///
    /// let scale = Vec2 {x : 5.0, y : 1.0};
    /// let mat = Mat3::scale (&scale);
    pub fn scale (scale: &Vec2) -> Mat3 {

        Mat3 {_value: [Vec3 {x: scale.x, y: 0.0,     z: 0.0},
                       Vec3 {x: 0.0,     y: scale.y, z: 0.0},
                       Vec3 {x: 0.0,     y: 0.0,     z: 1.0}]}
    }
}

/*================================================================================================*/
/*------MAT3 OPERATOR OVERLOADS-------------------------------------------------------------------*/
/*================================================================================================*/

impl Add for Mat3 {

    type Output = Mat3;

    // Addition operator (matrix)
    fn add (self, rhs: Mat3) -> Mat3 {

        Mat3 {_value: [self[0] + rhs[0],
                       self[1] + rhs[1],
                       self[2] + rhs[2]]}
    }
}

impl Add <Vec3> for Mat3 {

    type Output = Mat3;

    // Addition operator (vector)
    fn add (self, rhs: Vec3) -> Mat3 {

        Mat3 {_value: [self[0] + rhs,
                       self[1] + rhs,
                       self[2] + rhs]}
    }
}

impl AddAssign for Mat3 {

    // Addition assignment operator (matrix)
    fn add_assign (&mut self, rhs: Mat3) {

            self._value = (*self + rhs)._value;
    }
}

impl AddAssign <Vec3> for Mat3 {

    // Addition assignment operator (vector)
    fn add_assign (&mut self, rhs: Vec3) {

            self._value = (*self + rhs)._value;
    }
}

impl Sub for Mat3 {

    type Output = Mat3;

    // Subtraction operator (matrix)
    fn sub (self, rhs: Mat3) -> Mat3 {

        Mat3 {_value: [self[0] - rhs[0],
                       self[1] - rhs[1],
                       self[2] - rhs[2]]}
    }
}

impl Sub <Vec3> for Mat3 {

    type Output = Mat3;

    // Subtraction operator (vector)
    fn sub (self, rhs: Vec3) -> Mat3 {

        Mat3 {_value: [self[0] - rhs,
                       self[1] - rhs,
                       self[2] - rhs]}
    }
}

impl SubAssign for Mat3 {

    // Subtraction assignment operator (matrix)
    fn sub_assign (&mut self, rhs: Mat3) {

        self._value = (*self - rhs)._value;
    }
}

impl SubAssign <Vec3> for Mat3 {

    // Subtraction assignment operator (vector)
    fn sub_assign (&mut self, rhs: Vec3) {

            self._value = (*self - rhs)._value;
    }
}

impl Mul for Mat3 {

    type Output = Mat3;

    // Multiplication operator (matrix)
    fn mul (self, rhs: Mat3) -> Mat3 {

        let mut return_matrix = Mat3::new ();

        for row in 0..3 {

            for col in 0..3 {

                for inner in 0..3 {
                    return_matrix[row][col] += self[row][inner] * rhs[inner][col];
                }
            }
        }

        return_matrix
    }
}

impl Mul <Vec2> for Mat3 {

    type Output = Vec2;

    // Multiplication operator (Vec2)
    fn mul (self, rhs: Vec2) -> Vec2 {

        Vec2 {x : (self[0][0] * rhs.x) + (self[1][0] * rhs.y) + self[2][0],
              y : (self[0][1] * rhs.x) + (self[1][1] * rhs.y) + self[2][1]}
    }
}

impl MulAssign for Mat3 {

    // Multiplication assignment operator (matrix)
    fn mul_assign (&mut self, rhs: Mat3) {

        self._value = (*self * rhs)._value;
    }
}

impl PartialEq for Mat3 {

    // Equal to operator
    fn eq (&self, rhs: &Mat3) -> bool {

        self[0] == rhs[0] &&
        self[1] == rhs[1] &&
        self[2] == rhs[2]
    }

    // Not equal to operator
    fn ne (&self, rhs: &Mat3) -> bool {

        self[0] != rhs[0] ||
        self[1] != rhs[1] ||
        self[2] != rhs[2]
    }
}

impl Index <u8> for Mat3 {

    type Output = Vec3;

    // Index operator (immutable)
    fn index (&self, index: u8) -> &Vec3 {

        match index {

            0 => &self._value[0],
            1 => &self._value[1],
            2 => &self._value[2],
            _ => unreachable! ("Index out of range for Mat3")
        }
    }
}

impl IndexMut <u8> for Mat3 {

    // Index operator (mutable)
    fn index_mut (&mut self, index: u8) -> &mut Vec3 {

        match index {

            0 => &mut self._value[0],
            1 => &mut self._value[1],
            2 => &mut self._value[2],
            _ => unreachable! ("Index out of range for Mat3")
        }
    }
}

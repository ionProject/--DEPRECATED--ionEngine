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

use ::util::math::{Util, Vec3, Vec4};

use std::ops::*;
use std::cmp::PartialEq;

/*===============================================================================================*/
/*------MAT3 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// 4x4 Matrix
///
/// This struct represents a 4x4 matrix. It is normally used for 3D graphics transformations,
/// such as translation, rotation, and scaling.
#[derive (Copy, Clone, Default, Serialize, Deserialize)]
pub struct Mat4 {

    // Private
    _value: [Vec4; 4]
}

/*===============================================================================================*/
/*------MAT3 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl Mat4 {

    /// Formats the matrix as a string.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::Mat4;
    /// #
    /// let mat = Mat4::identity ();
    /// println! ("{}", mat.to_string ());
    /// ```
    /// ```c
    /// Output : [1, 0, 0, 0]
    ///          [0, 1, 0, 0]
    ///          [0, 0, 1, 0]
    ///          [0, 0, 0, 1]
    pub fn to_string (&self) -> String {

        format! ("[{}]\n[{}]\n[{}]\n[{}]", self[0].to_string (),
                                           self[1].to_string (),
                                           self[2].to_string (),
                                           self[3].to_string ())
    }

/*===============================================================================================*/
/*------MAT3 PUBLIC STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

    /// Creates a matrix with default values.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::Mat4;
    /// #
    /// let mat = Mat4::new (); // Returns a matrix with all zeros
    pub fn new () -> Mat4 {

        Mat4 {_value: [Vec4::new (); 4]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a new identity matrix
    pub fn identity () -> Mat4 {

        Mat4 {_value: [Vec4 {x: 1.0, y: 0.0, z: 0.0, w: 0.0},
                       Vec4 {x: 0.0, y: 1.0, z: 0.0, w: 0.0},
                       Vec4 {x: 0.0, y: 0.0, z: 1.0, w: 0.0},
                       Vec4 {x: 0.0, y: 0.0, z: 0.0, w: 1.0}]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the determinant of a matrix.
    /// This is used as part of the inverse matrix calculation.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::Mat4;
    /// #
    /// let mat = Mat4::identity ();
    /// println! ("Determinant = {}", Mat4::determinant (&mat));
    /// ```
    /// ```c
    /// Output : Determinant = 1.0
    pub fn determinant (m: &Mat4) -> f32 {

        m[0][3] * m[1][2] * m[2][1] * m[3][0] - m[0][2] * m[1][3] * m[2][1] * m[3][0] - m[0][3] * m[1][1] * m[2][2] * m[3][0] + m[0][1] * m[1][3] * m[2][2] * m[3][0] +
        m[0][2] * m[1][1] * m[2][3] * m[3][0] - m[0][1] * m[1][2] * m[2][3] * m[3][0] - m[0][3] * m[1][2] * m[2][0] * m[3][1] + m[0][2] * m[1][3] * m[2][0] * m[3][1] +
        m[0][3] * m[1][0] * m[2][2] * m[3][1] - m[0][0] * m[1][3] * m[2][2] * m[3][1] - m[0][2] * m[1][0] * m[2][3] * m[3][1] + m[0][0] * m[1][2] * m[2][3] * m[3][1] +
        m[0][3] * m[1][1] * m[2][0] * m[3][2] - m[0][1] * m[1][3] * m[2][0] * m[3][2] - m[0][3] * m[1][0] * m[2][1] * m[3][2] + m[0][0] * m[1][3] * m[2][1] * m[3][2] +
        m[0][1] * m[1][0] * m[2][3] * m[3][2] - m[0][0] * m[1][1] * m[2][3] * m[3][2] - m[0][2] * m[1][1] * m[2][0] * m[3][3] + m[0][1] * m[1][2] * m[2][0] * m[3][3] +
        m[0][2] * m[1][0] * m[2][1] * m[3][3] - m[0][0] * m[1][2] * m[2][1] * m[3][3] - m[0][1] * m[1][0] * m[2][2] * m[3][3] + m[0][0] * m[1][1] * m[2][2] * m[3][3]
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the inverse of a matrix.
    ///
    /// When multiplied by the original matrix, the end result will be an identity matrix.
    pub fn inverse (m: &Mat4) -> Mat4 {

        let det = 1.0 / Mat4::determinant (m);

        Mat4 {_value: [Vec4 {x: (m[1][2] * m[2][3] * m[3][1] - m[1][3] * m[2][2] * m[3][1] + m[1][3] * m[2][1] * m[3][2] - m[1][1] * m[2][3] * m[3][2] - m[1][2] * m[2][1] * m[3][3] + m[1][1] * m[2][2] * m[3][3]) * det,
                             y: (m[0][3] * m[2][2] * m[3][1] - m[0][2] * m[2][3] * m[3][1] - m[0][3] * m[2][1] * m[3][2] + m[0][1] * m[2][3] * m[3][2] + m[0][2] * m[2][1] * m[3][3] - m[0][1] * m[2][2] * m[3][3]) * det,
                             z: (m[0][2] * m[1][3] * m[3][1] - m[0][3] * m[1][2] * m[3][1] + m[0][3] * m[1][1] * m[3][2] - m[0][1] * m[1][3] * m[3][2] - m[0][2] * m[1][1] * m[3][3] + m[0][1] * m[1][2] * m[3][3]) * det,
                             w: (m[0][3] * m[1][2] * m[2][1] - m[0][2] * m[1][3] * m[2][1] - m[0][3] * m[1][1] * m[2][2] + m[0][1] * m[1][3] * m[2][2] + m[0][2] * m[1][1] * m[2][3] - m[0][1] * m[1][2] * m[2][3]) * det},

                       Vec4 {x: (m[1][3] * m[2][2] * m[3][0] - m[1][2] * m[2][3] * m[3][0] - m[1][3] * m[2][0] * m[3][2] + m[1][0] * m[2][3] * m[3][2] + m[1][2] * m[2][0] * m[3][3] - m[1][0] * m[2][2] * m[3][3]) * det,
                             y: (m[0][2] * m[2][3] * m[3][0] - m[0][3] * m[2][2] * m[3][0] + m[0][3] * m[2][0] * m[3][2] - m[0][0] * m[2][3] * m[3][2] - m[0][2] * m[2][0] * m[3][3] + m[0][0] * m[2][2] * m[3][3]) * det,
                             z: (m[0][3] * m[1][2] * m[3][0] - m[0][2] * m[1][3] * m[3][0] - m[0][3] * m[1][0] * m[3][2] + m[0][0] * m[1][3] * m[3][2] + m[0][2] * m[1][0] * m[3][3] - m[0][0] * m[1][2] * m[3][3]) * det,
                             w: (m[0][2] * m[1][3] * m[2][0] - m[0][3] * m[1][2] * m[2][0] + m[0][3] * m[1][0] * m[2][2] - m[0][0] * m[1][3] * m[2][2] - m[0][2] * m[1][0] * m[2][3] + m[0][0] * m[1][2] * m[2][3]) * det},

                       Vec4 {x: (m[1][1] * m[2][3] * m[3][0] - m[1][3] * m[2][1] * m[3][0] + m[1][3] * m[2][0] * m[3][1] - m[1][0] * m[2][3] * m[3][1] - m[1][1] * m[2][0] * m[3][3] + m[1][0] * m[2][1] * m[3][3]) * det,
                             y: (m[0][3] * m[2][1] * m[3][0] - m[0][1] * m[2][3] * m[3][0] - m[0][3] * m[2][0] * m[3][1] + m[0][0] * m[2][3] * m[3][1] + m[0][1] * m[2][0] * m[3][3] - m[0][0] * m[2][1] * m[3][3]) * det,
                             z: (m[0][1] * m[1][3] * m[3][0] - m[0][3] * m[1][1] * m[3][0] + m[0][3] * m[1][0] * m[3][1] - m[0][0] * m[1][3] * m[3][1] - m[0][1] * m[1][0] * m[3][3] + m[0][0] * m[1][1] * m[3][3]) * det,
                             w: (m[0][3] * m[1][1] * m[2][0] - m[0][1] * m[1][3] * m[2][0] - m[0][3] * m[1][0] * m[2][1] + m[0][0] * m[1][3] * m[2][1] + m[0][1] * m[1][0] * m[2][3] - m[0][0] * m[1][1] * m[2][3]) * det},

                       Vec4 {x: (m[1][2] * m[2][1] * m[3][0] - m[1][1] * m[2][2] * m[3][0] - m[1][2] * m[2][0] * m[3][1] + m[1][0] * m[2][2] * m[3][1] + m[1][1] * m[2][0] * m[3][2] - m[1][0] * m[2][1] * m[3][2]) * det,
                             y: (m[0][1] * m[2][2] * m[3][0] - m[0][2] * m[2][1] * m[3][0] + m[0][2] * m[2][0] * m[3][1] - m[0][0] * m[2][2] * m[3][1] - m[0][1] * m[2][0] * m[3][2] + m[0][0] * m[2][1] * m[3][2]) * det,
                             z: (m[0][2] * m[1][1] * m[3][0] - m[0][1] * m[1][2] * m[3][0] - m[0][2] * m[1][0] * m[3][1] + m[0][0] * m[1][2] * m[3][1] + m[0][1] * m[1][0] * m[3][2] - m[0][0] * m[1][1] * m[3][2]) * det,
                             w: (m[0][1] * m[1][2] * m[2][0] - m[0][2] * m[1][1] * m[2][0] + m[0][2] * m[1][0] * m[2][1] - m[0][0] * m[1][2] * m[2][1] - m[0][1] * m[1][0] * m[2][2] + m[0][0] * m[1][1] * m[2][2]) * det}]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a 3D translation matrix.
    ///
    /// It represents a transformation in 3D space.
    /// When combined with a rotation and scale matrix, it creates a model matrix.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::{Mat4, Vec3};
    /// #
    /// let pos = Vec3 {x : 10.0, y : 43.0, z : 29.0};
    /// let mat = Mat4::translate (&pos);
    pub fn translate (position: &Vec3) -> Mat4 {

        Mat4 {_value: [Vec4 {x: 1.0, y: 0.0, z: 0.0, w: position.x},
                       Vec4 {x: 0.0, y: 1.0, z: 0.0, w: position.y},
                       Vec4 {x: 0.0, y: 0.0, z: 1.0, w: position.z},
                       Vec4 {x: 0.0, y: 0.0, z: 0.0, w: 1.0}]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a 3D rotation matrix.
    ///
    /// It represents a rotation in 3D space.
    /// When combined with a translation and scale matrix, it creates a model matrix.
    ///
    /// It takes in a Vec3 argument, where the rotation is in degrees
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::{Mat4, Vec3};
    /// #
    /// let rot = Vec3 {x : 45.0, y : 0.0, z : 180.0};
    /// let mat = Mat4::rotate (&rot);
    pub fn rotate (rotation: &Vec3) -> Mat4 {

        // Convert rotation to radians
        let rad_rotation = Vec3 {x: Util::deg2rad (rotation.x),
                                 y: Util::deg2rad (rotation.y),
                                 z: Util::deg2rad (rotation.z)};

        let x_sin = rad_rotation.x.sin (); let x_cos = rad_rotation.x.cos ();
        let y_sin = rad_rotation.y.sin (); let y_cos = rad_rotation.y.cos ();
        let z_sin = rad_rotation.z.sin (); let z_cos = rad_rotation.z.cos ();

        // Transform x-axis
        let rotx = Mat4 {_value: [Vec4 {x: 1.0, y: 0.0,   z: 0.0,    w: 0.0},
                                  Vec4 {x: 0.0, y: x_cos, z: -x_sin, w: 0.0},
                                  Vec4 {x: 0.0, y: x_sin, z: x_cos,  w: 0.0},
                                  Vec4 {x: 0.0, y: 0.0,   z: 0.0,    w: 1.0}]};

        // Transform y-axis
        let roty = Mat4 {_value: [Vec4 {x: y_cos,  y: 0.0, z: y_sin, w: 0.0},
                                  Vec4 {x: 0.0,    y: 1.0, z: 0.0,   w: 0.0},
                                  Vec4 {x: -y_sin, y: 0.0, z: y_cos, w: 0.0},
                                  Vec4 {x: 0.0,    y: 0.0, z: 0.0,   w: 1.0}]};

        // Transform z-axis
        let rotz = Mat4 {_value: [Vec4 {x: z_cos, y: -z_sin, z: 0.0, w: 0.0},
                                  Vec4 {x: z_sin, y: z_cos,  z: 0.0, w: 0.0},
                                  Vec4 {x: 0.0,   y: 0.0,    z: 1.0, w: 0.0},
                                  Vec4 {x: 0.0,   y: 0.0,    z: 0.0, w: 1.0}]};

        rotz * roty * rotx
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a 3D scale matrix.
    ///
    /// It represents a scale in 3D space.
    /// When combined with a translation and rotation matrix, it creates a model matrix.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::util::math::{Mat4, Vec3};
    /// #
    /// let scale = Vec3 {x : 5.0, y : 1.0, z : 9.0};
    /// let mat = Mat4::scale (&scale);
    pub fn scale (scale: &Vec3) -> Mat4 {

        Mat4 {_value: [Vec4 {x: scale.x, y: 0.0,     z: 0.0,     w: 0.0},
                       Vec4 {x: 0.0,     y: scale.y, z: 0.0,     w: 0.0},
                       Vec4 {x: 0.0,     y: 0.0,     z: scale.z, w: 0.0},
                       Vec4 {x: 0.0,     y: 0.0,     z: 0.0,     w: 1.0}]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns an orthographic projection matrix
    pub fn ortho (left: f32, right: f32, top: f32, bottom: f32, z_near: f32, z_far: f32) -> Mat4 {

        Mat4 {_value: [Vec4 {x: 2.0 / (right - left), y: 0.0,                  z: 0.0,                     w: 0.0},
                       Vec4 {x: 0.0,                  y: 2.0 / (top - bottom), z: 0.0,                     w: 0.0},
                       Vec4 {x: 0.0,                  y: 0.0,                  z: -2.0 / (z_far - z_near), w: 0.0},
                       Vec4 {x: -(right + left)   / (right - left),
                             y: -(top + bottom)   / (top - bottom),
                             z: -(z_far + z_near) / (z_far - z_near),
                             w: 1.0}]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a perspective transformation matrix
    pub fn perspective (fov: f32, aspect: f32, near: f32, far: f32) -> Mat4 {

        let f = Util::cot (fov / 2.0);

        Mat4 {_value: [Vec4 {x: f / aspect, y: 0.0, z: 0.0,                               w: 0.0},
                       Vec4 {x: 0.0,        y: f,   z: 0.0,                               w: 0.0},
                       Vec4 {x: 0.0,        y: 0.0, z: (far + near) / (near - far),       w: -1.0},
                       Vec4 {x: 0.0,        y: 0.0, z: (2.0 * far * near) / (near - far), w: 0.0}]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a view matrix
    pub fn view (eye: &Vec3, target: &Vec3, up: &Vec3) -> Mat4 {

        let f = Vec3::normalize (&(*target - *eye));
        let s = Vec3::normalize (&Vec3::cross (&f, &up));
        let u = Vec3::cross (&s, &f);

        Mat4 {_value: [Vec4 {x: s.x,                   y: u.x,                   z: -f.x,                 w: 0.0},
                       Vec4 {x: s.y,                   y: u.y,                   z: -f.y,                 w: 0.0},
                       Vec4 {x: s.z,                   y: u.z,                   z: -f.z,                 w: 0.0},
                       Vec4 {x: -Vec3::dot (&eye, &s), y: -Vec3::dot (&eye, &u), z: Vec3::dot (&eye, &f), w: 1.0}]}
    }
}

/*===============================================================================================*/
/*------MAT3 OPERATOR OVERLOADS------------------------------------------------------------------*/
/*===============================================================================================*/

impl Add for Mat4 {

    type Output = Mat4;

    // Addition operator (matrix)
    fn add (self, rhs: Mat4) -> Mat4 {

        Mat4 {_value: [self[0] + rhs[0],
                       self[1] + rhs[1],
                       self[2] + rhs[2],
                       self[3] + rhs[3]]}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Add <Vec4> for Mat4 {

    type Output = Mat4;

    // Addition operator (vector)
    fn add (self, rhs: Vec4) -> Mat4 {

        Mat4 {_value: [self[0] + rhs,
                       self[1] + rhs,
                       self[2] + rhs,
                       self[3] + rhs]}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign for Mat4 {

    // Addition assignment operator (matrix)
    fn add_assign (&mut self, rhs: Mat4) {

            self._value = (*self + rhs)._value;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign <Vec4> for Mat4 {

    // Addition assignment operator (vector)
    fn add_assign (&mut self, rhs: Vec4) {

            self._value = (*self + rhs)._value;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub for Mat4 {

    type Output = Mat4;

    // Subtraction operator (matrix)
    fn sub (self, rhs: Mat4) -> Mat4 {

        Mat4 {_value: [self[0] - rhs[0],
                       self[1] - rhs[1],
                       self[2] - rhs[2],
                       self[3] - rhs[3]]}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub <Vec4> for Mat4 {

    type Output = Mat4;

    // Subtraction operator (vector)
    fn sub (self, rhs: Vec4) -> Mat4 {

        Mat4 {_value: [self[0] - rhs,
                       self[1] - rhs,
                       self[2] - rhs,
                       self[3] - rhs]}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign for Mat4 {

    // Subtraction assignment operator (matrix)
    fn sub_assign (&mut self, rhs: Mat4) {

        self._value = (*self - rhs)._value;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign <Vec4> for Mat4 {

    // Subtraction assignment operator (vector)
    fn sub_assign (&mut self, rhs: Vec4) {

            self._value = (*self - rhs)._value;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul for Mat4 {

    type Output = Mat4;

    // Multiplication operator (matrix)
    fn mul (self, rhs: Mat4) -> Mat4 {

        let mut m = Mat4::new ();

        for row in 0..4 {

            for col in 0..4 {

                for inner in 0..4 {
                    m[row][col] += rhs[row][inner] * self[inner][col];
                }
            }
        }

        m
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul <Vec3> for Mat4 {

    type Output = Vec3;

    // Multiplication operator (Vec3)
    fn mul (self, rhs: Vec3) -> Vec3 {

        Vec3 {x: (self[0][0] * rhs.x) + (self[1][0] * rhs.y) + (self[2][0] * rhs.z) + self[3][0],
              y: (self[0][1] * rhs.x) + (self[1][1] * rhs.y) + (self[2][1] * rhs.z) + self[3][1],
              z: (self[0][2] * rhs.x) + (self[1][2] * rhs.y) + (self[2][2] * rhs.z) + self[3][2]}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl MulAssign for Mat4 {

    // Multiplication assignment operator (matrix)
    fn mul_assign (&mut self, rhs: Mat4) {

        self._value = (*self * rhs)._value;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl PartialEq for Mat4 {

    // Equal to operator
    fn eq (&self, rhs: &Mat4) -> bool {

        self[0] == rhs[0] &&
        self[1] == rhs[1] &&
        self[2] == rhs[2] &&
        self[3] == rhs[3]
    }

/*-----------------------------------------------------------------------------------------------*/

    // Not equal to operator
    fn ne (&self, rhs: &Mat4) -> bool {

        self[0] != rhs[0] ||
        self[1] != rhs[1] ||
        self[2] != rhs[2] ||
        self[3] != rhs[3]
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Index <u8> for Mat4 {

    type Output = Vec4;

    // Index operator (immutable)
    fn index (&self, index: u8) -> &Vec4 {

        match index {

            0 => &self._value[0],
            1 => &self._value[1],
            2 => &self._value[2],
            3 => &self._value[3],
            _ => unreachable! ("Index out of range for Mat4")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl IndexMut <u8> for Mat4 {

    // Index operator (mutable)
    fn index_mut (&mut self, index: u8) -> &mut Vec4 {

        match index {

            0 => &mut self._value[0],
            1 => &mut self._value[1],
            2 => &mut self._value[2],
            3 => &mut self._value[3],
            _ => unreachable! ("Index out of range for Mat4")
        }
    }
}

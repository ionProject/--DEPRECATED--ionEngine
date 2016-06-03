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

/*================================================================================================*/
//! The math module.
//!
//! It contains various bits of mathematical functionality, including:
//! Vectors,
//! Matrices,
//! Quaternions,
//! Colours,
//! Bounding volumes,
//! Random numbers,
//! Utilities, etc.
/*================================================================================================*/

use std::f32::consts;

// Static variables
/// PI multipied by two
pub static TWO_PI  : f32 = consts::PI;
/// The value of PI
pub static PI      : f32 = consts::PI * 2.0;
/// Half of PI
pub static HALF_PI : f32 = consts::PI / 2.0;

// Modules
mod colour;
mod mat3;
mod mat4;
mod util;
mod vec2;
mod vec3;
mod vec4;

pub use self::colour::Colour;
pub use self::util::Util;
pub use self::mat3::Mat3;
pub use self::mat4::Mat4;
pub use self::vec2::Vec2;
pub use self::vec3::Vec3;
pub use self::vec4::Vec4;

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
/*------VERSION STRUCT---------------------------------------------------------------------------*/
/*===============================================================================================*/

/// A super simple struct that represents a version (major, minor, patch).
#[derive (Copy, Clone, PartialEq, PartialOrd)]
pub struct Version {

    // Public
    /// The major version
    pub major: i32,
    /// The minor version
    pub minor: i32,
    /// The patch version
    pub patch: i32
}

/*===============================================================================================*/
/*------VERSION PUBLIC METHODS-------------------------------------------------------------------*/
/*===============================================================================================*/

impl Version {

    /// Formats the version as a string
    pub fn to_string (&self) -> String {
        format! ("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

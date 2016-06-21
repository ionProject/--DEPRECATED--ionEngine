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

use ::engine::backend::{State, Type};

/*===============================================================================================*/
/*------PLUGIN STRUCT----------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used to store information on a backend plugin.
pub struct Plugin {

    // Public
    /// The name of the backend.
    pub name: String,
    /// The author.
    pub author: String,
    /// Description of the backend.
    pub description: String,
    /// Path to the plugin.
    pub path: String,
    /// The type of backend.
    pub backend_type: Type,
    /// The current state of the backend.
    pub state: State,
}

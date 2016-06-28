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
/*------CONFIG STRUCT----------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used to load the backend configuration.
#[derive (Serialize, Deserialize)]
pub struct Config {

    // Public
    /// The backend directory.
    pub backend_dir: String,
    /// The array of default backends.
    ///
    /// * `Index 0` - Audio backend.
    /// * `Index 1` - Rendering backend.
    /// * `Index 2` - Window backend.
    /// * `Index 3` - Input backend.
    pub default_backend: [String; 4],
}

/*===============================================================================================*/
/*------CONFIG PUBLIC STATIC METHODS-------------------------------------------------------------*/
/*===============================================================================================*/

impl Default for Config {

    fn default () -> Config {

        Config {

            backend_dir: "backend".to_string (),
            default_backend: ["None".to_string (),
                              "None".to_string (),
                              "None".to_string (),
                              "None".to_string ()],
        }
    }
}

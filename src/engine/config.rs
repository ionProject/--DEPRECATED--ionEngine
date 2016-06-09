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

extern crate serde;
extern crate serde_json;

/*===============================================================================================*/
/*------CONFIGMANAGER STRUCT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Handles the application config.
///
/// It is resposible for loading the app config, and allowing other modules easy access to that data.
/// It is also used for saving the app config.
#[derive (Clone, Default)]
pub struct ConfigManager {

    // Public
    /// The path to the config directory.
    pub config_dir: String
}

/*===============================================================================================*/
/*------CONFIGMANAGER PUBLIC STATIC METHODS------------------------------------------------------*/
/*===============================================================================================*/

impl ConfigManager {

    /// Returns a new instance of the config manager.
    ///
    /// # Default values
    /// * `config_dir` - "config".to_string ()
    ///
    /// # Return value
    /// A new config manager instance.
    pub fn new () -> ConfigManager {

        ConfigManager {config_dir: "config".to_string ()}
    }
}

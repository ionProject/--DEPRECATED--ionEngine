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
/*------CONFIGMANAGER STRUCT----------------------------------------------------------------------*/
/*================================================================================================*/

/// Handles the application config.
///
/// It is resposible for loading the app config, and allowing other modules easy access to that data.
/// It is also used for saving the app config.
#[derive (Clone)]
pub struct ConfigManager {

    // Public
    /// The path to the config file.
    pub config_file: String
}

/*================================================================================================*/
/*------CONFIGMANAGER PUBLIC MEMBERS--------------------------------------------------------------*/
/*================================================================================================*/

impl ConfigManager {

    /// Resets the config to it's default state.
    pub fn reset_config (&self) {
        unimplemented! ();
    }

    /// Loads values from the config file.
    pub fn load_config (&self) {
        unimplemented! ();
    }

    /// Saves values to the config file
    pub fn save_config (&self) {
        unimplemented! ();
    }

/*================================================================================================*/
/*------CONFIGMANAGER PUBLIC STATIC MEMBERS-------------------------------------------------------*/
/*================================================================================================*/

    /// Returns a new instance of the config manager.
    pub fn new (config_file_path: &str) -> ConfigManager {
        ConfigManager {config_file: config_file_path.to_string ()}
    }
}

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

extern crate glob;
extern crate serde;
extern crate serde_json;

use self::glob::glob;

use std::vec::Vec;

/*===============================================================================================*/
/*------CONFIG STRUCT----------------------------------------------------------------------------*/
/*===============================================================================================*/

#[derive (Clone)]
pub struct Config {

    // Public
    /// The name of the config file.
    pub name: String,
    /// The path to the config file.
    pub path: String
}

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
    pub config_dir: String,

    // Private
    _config_list: Vec<Config>
}

/*===============================================================================================*/
/*------CONFIGMANAGER PUBLIC METHODS-------------------------------------------------------------*/
/*===============================================================================================*/

impl ConfigManager {

    /// Queries the config directory, and stores a list of configs.
    pub fn query_config_dir (&mut self) {

        // Clear the old config list
        self._config_list.clear ();

        info! ("Searching for config files...");

        // Recurse through all items in the config directory
        for path in glob (&format! ("{}/*{}", &self.config_dir, ".json")).unwrap ().filter_map (Result::ok) {

            self._config_list.push (Config {name: path.file_stem ().unwrap ().to_str ().unwrap ().to_string (),
                                            path: path.to_str ().unwrap ().to_string ()});

            info! ("Found: {:?}", &path.file_name ().unwrap ());
        }

        // If list is empty, print empty
        // Otherwise, print searching complete
        if self._config_list.is_empty () {
            info! ("No config files found.");
        }

        else {
            info! ("Config searching complete.");
        }
    }

/*===============================================================================================*/
/*------CONFIGMANAGER PUBLIC STATIC METHODS------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new instance of the config manager.
    ///
    /// # Default values
    /// * `config_dir` - "config".to_string ()
    ///
    /// # Return value
    /// A new config manager instance.
    pub fn new () -> ConfigManager {

        ConfigManager {config_dir: "config".to_string (),
                       _config_list: Vec::new ()}
    }
}

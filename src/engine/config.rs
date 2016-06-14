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
use self::serde::{Serialize, Deserialize};

use std::io::{Read, Write};
use std::fs::File;
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

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a new config with given name.
    pub fn create_config<T: Serialize + Deserialize + Default> (&mut self, config_name: &str) {

        debug! ("Creating config file with name: '{}'", config_name);

        let mut f = File::create (format! ("{}/{}.json", &self.config_dir, config_name)).unwrap ();
        f.write (serde_json::to_string_pretty (&T::default ()).unwrap ().as_bytes ()).unwrap ();

        self._config_list.push (Config {name: config_name.to_string (),
                                        path: format! ("{}/{}.json", &self.config_dir, config_name)});
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Loads a config of a given name.
    ///
    /// The config file is then converted into the type `T`. `T` must derive both the
    /// [`Serialize`] (http://serde-rs.github.io/serde/serde/ser/trait.Serialize.html) and
    /// [`Deserialize`] (http://serde-rs.github.io/serde/serde/de/trait.Deserialize.html) traits.
    ///
    /// If this fails, an [`Err`] (https://doc.rust-lang.org/std/result/) is returned.
    ///
    /// # Return value
    /// A result containing the config struct `T`.
    pub fn load_config<T: Serialize + Deserialize> (&self, config_name: &str) -> Result<T, ()> {

        debug! ("Loading config file with name: '{}'", config_name);

        for item in &self._config_list {

            if item.name == config_name {

                let mut f = File::open (&item.path).unwrap ();
                let mut s = String::new ();

                f.read_to_string (&mut s).unwrap ();
                return Ok (serde_json::from_str::<T> (&s).unwrap ());
            }
        }

        warn! ("Could not find config file: '{}'.
                Try creating it first.", config_name);
        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Saves a config of a given name.
    pub fn save_config<T: Serialize + Deserialize> (&self, config_name: &str, data: &T) -> Result<(), ()> {

        debug! ("Saving config file with name: '{}'", config_name);

        for item in &self._config_list {

            if item.name == config_name {

                let mut f = File::create (&item.path).unwrap ();
                f.write (serde_json::to_string_pretty (&data).unwrap ().as_bytes ()).unwrap ();
                return Ok (())
            }
        }

        warn! ("Could not find config file: '{}'.
                Try creating it first.", config_name);
        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Check if a config file exists.
    pub fn config_exists (&self, config_name: &str) -> bool {

        for item in &self._config_list {

            if item.name == config_name {
                return true;
            }
        }

        false
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

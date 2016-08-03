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
extern crate tar;

use ::engine::App;
use ::util::serialization::{Deserializer, Serializer};

use self::serde::{Serialize, Deserialize};
use self::tar::Archive;

use std::fs::File;
use std::path::Path;

/*===============================================================================================*/
/*------CONFIG LOADER STRUCT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Allows the loading of configuration files.
#[derive (Copy, Clone)]
pub struct ConfigLoader;

/*===============================================================================================*/
/*------CONFIG LOADER PUBLIC METHODS-------------------------------------------------------------*/
/*===============================================================================================*/

impl ConfigLoader {
    
    /// Initializes the config loader
    pub fn init (&self) {

        info! ("Unpacking default config files.");

        // Open the config file path
        let cfg_dir = &App::get_instance ().unwrap ().cfg_dir;
        let cfg_pkg = format! ("{}cfg.respkg", &App::get_instance ().unwrap ().res_dir);

        // Open the file location
        match File::open (&cfg_pkg) {

            Ok (a) => {

                let mut archive = Archive::new (a);
                
                match archive.entries () {

                    Ok (entries) => {

                        // Iterate through the entries
                        for file in entries {

                            // Get file name and calc final path.
                            let mut f    = file.unwrap ();
                            let f_name   = f.path ().unwrap ().into_owned ();
                            let out_path = format! ("{}{}", cfg_dir, f_name.display ());

                            // Check if the config file already exists.
                            if Path::new (&out_path).exists () {
                                continue;
                            }

                            // Unpack the file to the directory
                            info! ("Unpacking \"{}\" to \"{}\".", f_name.display (), out_path);
                            f.unpack (out_path).unwrap ();
                        }
                    },

                    Err (e) => warn! ("Could not get resource entries in \"{}\".\n{}", &cfg_pkg, e)
                }
            },

            Err (e) => warn! ("Could not open resource package \"{}\".\n{}", cfg_pkg, e)
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Creates a new config file.
    pub fn new_config<T: Default + Serialize> (&self, cfg_dir: &str, config_name: &str) -> Result<(), ()> {

        let cfg_path = &format! ("{}{}.cfg", cfg_dir, config_name);
        info! ("Creating new config file \"{}\".", cfg_path);

        match Serializer::to_file::<T> (&T::default (), cfg_path) {

            Ok (_) => Ok (()),
            Err (e) => {

                error! ("Could not create config file \"{}\".\n{}", cfg_path, e);
                Err (())
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Loads the config file of a given name.
    pub fn load_config<T: Deserialize> (&self, cfg_dir: &str, config_name: &str) -> Result<T, ()> {

        let cfg_path = &format! ("{}{}.cfg", cfg_dir, config_name);
        info! ("Loading config file \"{}\".", cfg_path);
        
        match Deserializer::from_file::<T> (cfg_path) {

            Ok (config) => Ok (config),
            Err (e) => {

                error! ("Config file \"{}\" could not be loaded.\n{}", cfg_path, e);
                Err (())
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Saves the config file of a given name.
    pub fn save_config<T: Serialize> (&self, cfg_dir: &str, config_name: &str, config_data: &T) -> Result<(), ()> {

        let cfg_path = &format! ("{}{}.cfg", cfg_dir, config_name);
        info! ("Saving config file \"{}\".", cfg_path);

        match Serializer::to_file::<T> (config_data, cfg_path) {

            Ok (_) => Ok (()),
            Err (e) => {

                error! ("Could not save to config file \"{}\".\n{}", cfg_path, e);
                Err (())
            }
        }
    }
}

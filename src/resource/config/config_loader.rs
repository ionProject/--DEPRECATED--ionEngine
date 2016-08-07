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
extern crate zip;

use ::util::Directory;
use ::util::serialization::{Deserializer, Serializer};

use self::serde::{Serialize, Deserialize};
use self::zip::ZipArchive;

use std::fs::File;
use std::io;
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

        info! ("Copying default config files.");

        // Open the persistent config path
        let p_config_dir = Directory::get_persistent_config_directory ();
        let config_pkg   = format! ("{}cfg.respkg", Directory::get_resource_directory ());

        // Open the file location
        match File::open (&config_pkg) {

            Ok (a) => {

                // Open the zip archive
                match ZipArchive::new (a) {

                    Ok (mut archive) => {

                        // Iterate through the zip file entries
                        for i in 0..archive.len () {

                            // Get the file and set ouput path
                            let mut file = archive.by_index (i).unwrap ();
                            let out_path = format! ("{}{}", p_config_dir, file.name ());

                            // Check if the config file already exists.
                            if Path::new (&out_path).exists () {
                                continue;
                            }

                            // Unpack the file to the directory
                            info! ("Unpacking \"{}\" to \"{}\".", file.name (), out_path);
                            
                            // Unpack the file to the output path
                            let mut out_file = File::create (&out_path).unwrap ();
                            io::copy (&mut file, &mut out_file).unwrap ();
                        }
                    },

                    Err (e) => warn! ("Could not open resource package \"{}\".\n{}", config_pkg, e)
                }
            },

            Err (e) => warn! ("Could not open resource package \"{}\".\n{}", config_pkg, e)
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

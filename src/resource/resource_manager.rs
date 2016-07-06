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

use ::resource::{ResourceConfig, ResourceDirectory};
use ::resource::config::ConfigLoader;
use ::util::serialization::Deserializer;

/*===============================================================================================*/
/*------RESOURCE MANAGER STRUCT------------------------------------------------------------------*/
/*===============================================================================================*/

/// Interface for resource loading and management.
#[derive (Clone, Default)]
pub struct ResourceManager {

    // Public
    /// The path to the config directory.
    pub default_config_path: String,
    /// The config loader.
    pub config_loader: ConfigLoader,

    // Private
    _config: ResourceConfig,
}

/*===============================================================================================*/
/*------RESOURCE MANAGER PUBLIC METHODS----------------------------------------------------------*/
/*===============================================================================================*/

impl ResourceManager {

    /// Initializes the Resource Manager.
    pub fn init (&mut self) {

        // Get the resource config from the config path.
        let config_path = format! ("{}resource.cfg", &self.default_config_path);
        info! ("Loading \"{}\".", &config_path);

        // Load and store the resource manager config.
        match Deserializer::from_file::<ResourceConfig> (&config_path) {

            Ok (config) => {
                
                info! ("Loaded \"{}\".", &config_path);
                self._config = config;
            },
            
            Err (e) => {

                error! ("Failed to load \"{}\"\n{}\nThings will not work as expected.", &config_path, e);
                return;
            }
        };

        // Get list of config files
        self.config_loader._config_paths = self.get_paths_with_tag ("config");
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns all paths with the specified tag.
    pub fn get_paths_with_tag (&self, tag: &str) -> Vec<ResourceDirectory> {

        let mut return_vec = Vec::new ();

        for path in &self._config.resource_dir_list {

            for path_tag in &path.directory_tags {

                if path_tag == tag {

                    return_vec.push (path.clone ());
                    break;
                }
            }
        }

        return_vec
    }

/*===============================================================================================*/
/*------RESOURCE MANAGER PUBLIC STATIC METHODS---------------------------------------------------*/
/*===============================================================================================*/

    /// Create a new instance of the Resource Manager.
    pub fn new () -> ResourceManager {

        ResourceManager {

            default_config_path: "config/".to_string (),
            config_loader: ConfigLoader::default (),
            _config: ResourceConfig::default ()
        }
    }
}

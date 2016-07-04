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

use ::resource::ResourceConfig;
use ::resource::config::ConfigLoader;
use ::util::serialization::Deserializer;

use std::cell::RefCell;
use std::rc::Rc;

/*===============================================================================================*/
/*------RESOURCE MANAGER STRUCT------------------------------------------------------------------*/
/*===============================================================================================*/

/// Interface for resource loading and management.
#[derive (Clone)]
pub struct ResourceManager {

    // Public
    /// The path to the config directory.
    pub config_path: String,

    // Private
    _config: ResourceConfig,
    _config_loader: Rc<RefCell<ConfigLoader>>
}

/*===============================================================================================*/
/*------RESOURCE MANAGER PUBLIC METHODS----------------------------------------------------------*/
/*===============================================================================================*/

impl ResourceManager {

    /// Loads the resource manager config, as well as the config for all resource loaders.
    pub fn load_config (&mut self) {

        // Get the resource config from the config path.
        let config_path = format! ("{}resource.cfg", &self.config_path);

        // Load and store the resource manager config.
        match Deserializer::from_file::<ResourceConfig> (&config_path) {

            Ok  (config) => self._config = config,
            Err (e) => error! ("Failed to load \"{}\"\n{}\nThings will not work as expected.", &config_path, e)
        };
    }

/*===============================================================================================*/
/*------RESOURCE MANAGER PUBLIC STATIC METHODS---------------------------------------------------*/
/*===============================================================================================*/

    /// Create a new instance of the Resource Manager.
    pub fn new () -> ResourceManager {

        ResourceManager {

            config_path: "resource.cfg".to_string (),
            _config: ResourceConfig::default (),
            _config_loader: Rc::new (RefCell::new (ConfigLoader::default ()))
        }
    }
}

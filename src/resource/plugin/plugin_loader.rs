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

use ::resource::ResourceManager;
use ::resource::plugin::PluginConfig;

/*===============================================================================================*/
/*------PLUGIN LOADER STRUCT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Allows for the loading of the various plugins.
pub struct PluginLoader {

    // Restricted
    /// Stores the plugin configuration.
    pub (resource) _plug_config: PluginConfig,

    // Private
    _plug_ext: String,
}

/*===============================================================================================*/
/*------PLUGIN LOADER PUBLIC METHODS-------------------------------------------------------------*/
/*===============================================================================================*/

impl PluginLoader {

    /// Initializes the plugin loader.
    pub fn init (&mut self, resource_manager: &ResourceManager) {

        if let Ok (config) = resource_manager.load_config::<PluginConfig> ("plugins") {
            self._plug_config = config;
        }
    }

/*===============================================================================================*/
/*------PLUGIN LOADER PUBLIC STATIC METHODS------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new instance of the plugin loader.
    pub fn new () -> PluginLoader {

        // Set the plugin extension 
        let plug_ext = if cfg! (target_os = "windows") {".dll"} 
                       else if cfg! (target_os = "linux") {".so"} 
                       else if cfg! (target_os = "macos") {".dylib"} 
                       else {panic! ("Platform unsupported")};

        PluginLoader {_plug_ext: plug_ext.to_string (),
                      _plug_config: PluginConfig::default ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for PluginLoader {

    fn default () -> PluginLoader {
        PluginLoader::new ()
    }
}

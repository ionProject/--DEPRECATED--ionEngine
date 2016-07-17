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

extern crate libloading;

use ::resource::ResourceManager;
use ::resource::plugin::PluginConfig;

use self::libloading::{Library, Symbol};

/*===============================================================================================*/
/*------PLUGIN LOADER STRUCT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Allows for the loading of the various plugins.
pub struct PluginLoader {

    // Private
    _plug_ext: String,
    _plug_config: PluginConfig,
}

/*===============================================================================================*/
/*------PLUGIN LOADER PUBLIC METHODS-------------------------------------------------------------*/
/*===============================================================================================*/

impl PluginLoader {

    /// Initializes the plugin loader.
    pub fn init (&mut self, resource_manager: &ResourceManager) {

        // Load the config file
        if let Ok (config) = resource_manager.load_config::<PluginConfig> ("plugins") {
            self._plug_config = config;
        }

        // Register the plugins in the list
        for plugin in &self._plug_config.plugin_list {
            self.register_plugin (plugin);
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Registers a plugin with the plugin loader.
    pub fn register_plugin (&self, plugin_path: &str) {

        // Open the library
        match Library::new (plugin_path) {

            Ok (lib) => {

                // Get the register function
                let register: Symbol<unsafe extern fn ()> = unsafe {

                    match lib.get (b"register\0") {

                        Ok (f) => f,
                        Err (e) => {

                            warn! ("Could not find function \"register\" in plugin \"{}\".\n{}", plugin_path, e);
                            return;
                        }
                    }
                };

                // Call the register function
                unsafe {register ();}
            },

            Err (e) => warn! ("Could not load plugin \"{}\".\n{}", plugin_path, e)
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

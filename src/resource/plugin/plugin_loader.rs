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

use ::engine::App;
use ::resource::ResourceManager;
use ::resource::plugin::PluginConfig;
use ::resource::{PluginInfo, PluginType};
use ::window::traits::WindowFactory;
use ::util::Directory;

use self::libloading::{Library, Symbol};

use std::path::Path;

/*===============================================================================================*/
/*------PLUGIN LOADER STRUCT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Allows for the loading of the various plugins.
pub struct PluginLoader {

    // Private
    _plug_ext: String,
    _plug_config: PluginConfig,
    _plug_list: Vec<Library>
}

/*===============================================================================================*/
/*------PLUGIN LOADER PUBLIC METHODS-------------------------------------------------------------*/
/*===============================================================================================*/

impl PluginLoader {

    /// Initializes the plugin loader.
    pub fn init (&mut self, resource_mgr: &ResourceManager) {

        // Load the config file
        if let Ok (config) = resource_mgr.load_config::<PluginConfig> ("plugins") {
            self._plug_config = config;
        }

        else {

            match resource_mgr.new_config::<PluginConfig> ("plugins") {
                Ok (_) | Err (_) => {}
            }
        }

        info! ("Registering plugins.");

        let plugin_dir = Directory::get_plugin_directory ();

        // Register the plugins in the list
        for plugin in &self._plug_config.plugin_list {

            let lib = self.register_plugin (&format! ("{}{}{}", plugin_dir, plugin, self._plug_ext));

            if let Some (s) = lib {
                self._plug_list.push (s)
            };
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Registers a plugin with the plugin loader.
    pub fn register_plugin (&self, plugin_path: &str) -> Option<Library> {

        // Check if plugin exists
        if !Path::new (plugin_path).exists () {

            warn! ("Plugin \"{}\" does not exist.", plugin_path);
            return None;
        }

        // Get the plugin type
        let plug_type = match self.get_plugin_info (plugin_path) {

            Ok (info) => info.plug_type,
            Err (_) => return None
        };

        // Open the library
        match Library::new (plugin_path) {

            Ok (lib) => {

                // From the plugin type, call the correct function
                match plug_type {

                    PluginType::WindowBackend => return self._load_plugin_window_backend (plugin_path, lib),
                }
            }

            Err (e) => warn! ("Could not load plugin \"{}\".\n{}", plugin_path, e)
        };

        None
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Retrieves information on a plugin.
    pub fn get_plugin_info (&self, plugin_path: &str) -> Result<PluginInfo, ()> {

        // Check if plugin exists
        if !Path::new (plugin_path).exists () {

            warn! ("Plugin \"{}\" does not exist.", plugin_path);
            return Err (());
        }

        // Open the library
        match Library::new (plugin_path) {

            Ok (lib) => {

                // Get the get info function
                let get_plugin_info: Symbol<unsafe extern fn () -> PluginInfo> = unsafe {

                    match lib.get (b"get_plugin_info\0") {

                        Ok (f) => f,
                        Err (e) => {

                            warn! ("Could not find function \"get_plugin_info\" in plugin \"{}\".\n{}", plugin_path, e);
                            return Err (());
                        }
                    }
                };

                // Call the get info function
                return Ok (unsafe {get_plugin_info ()});
            }

            Err (e) => warn! ("Could not load plugin \"{}\".\n{}", plugin_path, e)
        }

        Err (())
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
                      _plug_config: PluginConfig::default (),
                      _plug_list: Vec::new ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for PluginLoader {

    fn default () -> PluginLoader {
        PluginLoader::new ()
    }
}

/*===============================================================================================*/
/*------PLUGIN LOADER PRIVATE METHODS------------------------------------------------------------*/
/*===============================================================================================*/

impl PluginLoader {

    // Loads the window plugin.
    fn _load_plugin_window_backend (&self, plugin_path: &str, lib: Library) -> Option<Library> {

        // Get the get factory function
        {
            let get_factory: Symbol<unsafe extern fn () -> Box<WindowFactory>> = unsafe {

                match lib.get (b"get_factory\0") {

                    Ok (f) => f,
                    Err (e) => {

                        warn! ("Could not find function \"get_factory\" in plugin \"{}\".\n{}", plugin_path, e);
                        return None;
                    }
                }
            };

            let window_mgr = App::get_instance ().unwrap ().window_mgr.clone ();
            window_mgr.borrow_mut ().register_plugin (unsafe {get_factory ()});
        }

        // Call the register plugin and add library to list
        info! ("Registered plugin \"{}\"", plugin_path);
        Some (lib)
    }
}

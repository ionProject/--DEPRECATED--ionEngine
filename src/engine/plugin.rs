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

extern crate glob;
extern crate libloading;

use self::glob::glob;
use self::libloading::{Library, Symbol};

use std::vec::Vec;

/*================================================================================================*/
/*------PLUGINTYPE ENUM---------------------------------------------------------------------------*/
/*================================================================================================*/

/// The Plugin Type enum.
#[derive (Copy, Clone, PartialEq, Debug)]
pub enum PluginType {

    /// Used by audio backends.
    AudioBackend,
    /// Used by rendering backends.
    RenderBackend
}

/*================================================================================================*/
/*------PLUGIN STRUCT-----------------------------------------------------------------------------*/
/*================================================================================================*/

/// Stores the details of a plugin.
#[derive (Clone)]
pub struct Plugin {

    // Public
    /// The name of the plugin.
    pub name: String,
    /// The author of the plugin.
    pub author: String,
    /// A brief description of the plugin.
    pub description: String,
    /// The path to the plugin
    pub path: String,
    /// The type of plugin
    pub plugin_type: PluginType,
}

/*================================================================================================*/
/*------PLUGINMANAGER STRUCT----------------------------------------------------------------------*/
/*================================================================================================*/

#[derive (Clone)]
/// Manages the finding and loading of plugins.
pub struct PluginManager {

    // Public
    /// The list of plugins
    pub plugin_list: Vec<Plugin>,
    /// The plugin directory
    pub plugin_dir: String,

    // Private
    _plugin_ext: String
}

/*================================================================================================*/
/*------PLUGINMANAGER PUBLIC MEMBERS--------------------------------------------------------------*/
/*================================================================================================*/

impl PluginManager {

    /// Returns a new plugin manager
    pub fn new (plugin_dir: &str) -> PluginManager {

        // Set the platform extension
        let plug_ext = if cfg! (target_os = "windows") {".dll"}
                       else if cfg! (target_os = "linux") {".so"}
                       else if cfg! (target_os = "macos") {".dylib"}
                       else {panic! ("Platform unsupported")};

        PluginManager {plugin_list: Vec::new (),
                       plugin_dir: plugin_dir.to_owned (),
                       _plugin_ext: plug_ext.to_owned ()}
    }

    /// Queries the plugin directory, and stores a list of plugins
    pub fn query_plugins (&mut self) -> &Vec<Plugin> {

        // Clear the old plugin list
        self.plugin_list.clear ();

        // Recurse through all items in the plugin directory
        for path in glob (&format! ("{}/*{}", &self.plugin_dir, &self._plugin_ext)).unwrap ().filter_map (Result::ok) {

            // Load the library, and get function symbols
            let lib = Library::new (&path).unwrap ();

            let get_name: Symbol <unsafe extern fn () -> String> = unsafe {
                lib.get (b"get_name\0").unwrap ()
            };

            let get_author: Symbol <unsafe extern fn () -> String> = unsafe {
                lib.get (b"get_author\0").unwrap ()
            };

            let get_description: Symbol <unsafe extern fn () -> String> = unsafe {
                lib.get (b"get_description\0").unwrap ()
            };

            let get_type: Symbol <unsafe extern fn () -> PluginType> = unsafe {
                lib.get (b"get_type\0").unwrap ()
            };

            unsafe {

                // Add the plugin to the list
                self.plugin_list.push (Plugin {name: get_name (),
                                               author: get_author (),
                                               description: get_description (),
                                               path: path.to_str ().unwrap ().to_owned (),
                                               plugin_type: get_type ()});
            }
        }

        &self.plugin_list
    }
}

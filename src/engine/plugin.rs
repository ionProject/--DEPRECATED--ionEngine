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
#[derive (Copy, Clone, PartialEq)]
pub enum PluginType {

    /// Used by audio backends.
    AudioBackend,
    /// Used by rendering backends.
    RenderBackend
}

/*================================================================================================*/
/*------PLUGINMANAGER STRUCT----------------------------------------------------------------------*/
/*================================================================================================*/

#[derive (Clone)]
/// Manages the finding and loading of plugins.
pub struct PluginManager {

    // Private
    _plugin_list: Vec<String>,
    _plugin_dir: String
}

/*================================================================================================*/
/*------PLUGINMANAGER PUBLIC MEMBERS--------------------------------------------------------------*/
/*================================================================================================*/

impl PluginManager {

    /// Returns all plugins of a given type
    pub fn get_plugins_of_type (&self, plugin_type: PluginType) -> Vec<String> {

        let mut p_vec = Vec::<String>::new ();

        // Recurse through all items in the plugin directory
        for path in glob (&self._plugin_dir).unwrap ().filter_map (Result::ok) {

            // Load the library, the 'get_type' function, and the 'get_name' function
            let lib = Library::new (&path).unwrap ();

            let get_type: Symbol<unsafe extern fn () -> PluginType> = unsafe {
                lib.get (b"get_type\0").unwrap ()
            };

            let get_name: Symbol<unsafe extern fn () -> String> = unsafe {
                lib.get (b"get_name\0").unwrap ()
            };

            unsafe {

                // If the lib type equals the requested type, add it to the list
                if get_type () == plugin_type {
                    p_vec.push (get_name ());
                }
            }
        }

        p_vec
    }
}

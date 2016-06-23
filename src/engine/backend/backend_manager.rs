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
extern crate libloading;

use ::engine::App;
use ::engine::backend::{Config, Plugin, State, Type};
use ::util::Version;

use self::glob::glob;
use self::libloading::{Library, Symbol};

use std::fs;

/*===============================================================================================*/
/*------MANAGER STRUCT---------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Manages the loading of the various backend systems.
#[derive (Default)]
pub struct Manager {

    // Private
    _config: Config,
    _backend_list: Vec<Plugin>,
    _ext: String,
}

/*===============================================================================================*/
/*------MANAGER PUBLIC METHODS-------------------------------------------------------------------*/
/*===============================================================================================*/

impl Manager {

    /// Loads the configuration file.
    pub fn load_config (&mut self) {

        // Get a pointer to the config manager
        let cfg_mgr = App::get_config_manager ().unwrap ();
        let cfg_exists = cfg_mgr.borrow ().config_exists ("backend_manager");
        
        // Check if the config file for the backend manager exists.
        // If so, load it.
        // If not, create it.
        let cfg = {
        
            if cfg_exists {
                cfg_mgr.borrow ().load_config::<Config> ("backend_manager").unwrap ()
            }

            else {

                cfg_mgr.borrow_mut ().create_config::<Config> ("backend_manager");
                cfg_mgr.borrow ().load_config::<Config> ("backend_manager").unwrap ()
            }
        };

        self._config = cfg;
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Queries the backend directory for plugins.
    ///
    /// All valid backend plugins are then registered with the manager for loading.
    /// Any plugins with errors are ignored.
    pub fn query_backend_dir (&mut self) {

        // Clear the old backend list
        self._backend_list.clear ();
        info! ("Searching for backend plugins...");

        if !&self._check_backend_dir_exists () {
            return;
        }
        
        // Recurse through the backend directory, and get all backend plugins
        for path in glob (&format! ("{}/*{}", &self._config.backend_dir, &self._ext)).unwrap ().filter_map (Result::ok) {

            info! ("Found: {:?}", &path.file_name ().unwrap ());

            // Load the library and get the function symbols.
            match Library::new (&path) {

                Ok (lib) => {

                    // Plugin name
                    let get_name: Symbol<unsafe extern fn () -> String> = unsafe {

                        match lib.get (b"get_name\0") {

                            Ok (l) => l,
                            Err (e) => {

                                warn! ("Could not find function 'get_name' in library {:?}\n{}",
                                       &path.file_name ().unwrap (), e);
                                continue;
                            }
                        }
                    };

                    // Plugin author
                    let get_author: Symbol<unsafe extern fn () -> String> = unsafe {

                        match lib.get (b"get_author\0") {

                            Ok (l) => l,
                            Err (e) => {

                                warn! ("Could not find function 'get_author' in library {:?}\n{}",
                                       &path.file_name ().unwrap (), e);
                                continue;
                            }
                        }
                    };

                    // Plugin version
                    let get_version: Symbol<unsafe extern fn () -> Version> = unsafe {

                        match lib.get (b"get_version\0") {

                            Ok (l) => l,
                            Err (e) => {

                                warn! ("Could not find function 'get_version' in library {:?}\n{}",
                                       &path.file_name ().unwrap (), e);
                                continue;
                            }
                        }
                    };

                    // Plugin description
                    let get_description: Symbol<unsafe extern fn () -> String> = unsafe {

                        match lib.get (b"get_description\0") {

                            Ok (l) => l,
                            Err (e) => {

                                warn! ("Could not find function 'get_description' in library {:?}\n{}",
                                       &path.file_name ().unwrap (), e);
                                continue;
                            }
                        }
                    };

                    // Plugin type
                    let get_type: Symbol<unsafe extern fn () -> Type> = unsafe {

                        match lib.get (b"get_type\0") {

                            Ok (l) => l,
                            Err (e) => {

                                warn! ("Could not find function 'get_type' in library {:?}\n{}",
                                       &path.file_name ().unwrap (), e);
                                continue;
                            }
                        }
                    };

                    // Create a new instance of the plugin, and add it to the list
                    unsafe {

                        self._backend_list.push (Plugin {name: get_name (),
                                                         author: get_author (),
                                                         version: get_version (),
                                                         description: get_description (),
                                                         path: path.to_str ().unwrap ().to_string (),
                                                         b_type: get_type (),
                                                         state: State::Unloaded});
                    }

                    info! ("Added: {:?}", &path.file_name ().unwrap ());
                },

                Err (e) => {

                    warn! ("Could not load library {:?}\n{}", &path.file_name ().unwrap (), e);
                    continue;
                }
            }
        }

        // If list is empty, print empty
        // Otherwise, print searching complete
        if self._backend_list.is_empty () {
            info! ("No backend plugins found.");
        }

        else {
            info! ("Backend searching complete.");
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Lists all avaliable backends of a given type.
    pub fn list_backends (&self, backend_type: Type) -> Vec<String> {

        let mut return_vec = Vec::<String>::new ();

        for backend in &self._backend_list {

            if backend.b_type == backend_type {
                return_vec.push (backend.name.clone ());
            }
        }

        return_vec
    }

/*===============================================================================================*/
/*------MANAGER PUBLIC STATIC METHODS------------------------------------------------------------*/
/*===============================================================================================*/

    /// Creates a new instance of the BackendManager.
    ///
    /// # Return value
    /// A new instance of the backend manager.
    pub fn new () -> Manager {

        // Set the platform extension 
        let plug_ext = if cfg! (target_os = "windows") {".dll"} 
                       else if cfg! (target_os = "linux") {".so"} 
                       else if cfg! (target_os = "macos") {".dylib"} 
                       else {panic! ("Platform unsupported")};
        Manager {

            _config: Config::default (),
            _backend_list: Vec::new (),
            _ext: plug_ext.to_string ()
        }
    }

/*===============================================================================================*/
/*------MANAGER PRIVATE METHODS------------------------------------------------------------------*/
/*===============================================================================================*/

    // Checks if the backend directory exists.
    // If not, a new one is created.
    fn _check_backend_dir_exists (&self) -> bool {

        if fs::metadata (&self._config.backend_dir).is_err () {

            warn! ("Backend directory does not exist. Creating one now.");

            // Create new directory and check for errors
            match fs::create_dir (&self._config.backend_dir) {

                Ok  (_) => {

                    info! ("Directory creation successful.");
                    info! ("No backend plugins found.");
                    return false;
                },

                Err (e) => {

                    error! ("Failed to create backend directory: {}", e);
                    panic! ();
                }
            }
        }

        true
    }
}

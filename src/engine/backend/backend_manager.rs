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

//use ::engine::App;
use ::engine::backend::{Config, Info, Type, State};
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
    _backend_list: Vec<Info>,
    _ext: String,
}

/*===============================================================================================*/
/*------MANAGER PUBLIC METHODS-------------------------------------------------------------------*/
/*===============================================================================================*/

impl Manager {

    /// Loads the configuration file.
    pub fn load_config (&mut self) {

        // Get a pointer to the config manager
        /*let cfg_mgr = App::get_config_manager ().unwrap ();
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

        self._config = cfg;*/
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Saves the configuration file.
    pub fn save_config (&self) {

        // Get a pointer to the config manager
        /*let cfg_mgr = App::get_config_manager ().unwrap ();
        let cfg_exists = cfg_mgr.borrow ().config_exists ("backend_manager");

        // Check if the config file for the backend manager exists.
        // If so, save it.
        // If not, create it.
        if cfg_exists {
            cfg_mgr.borrow ().save_config ("backend_manager", &self._config).unwrap ();
        }

        else {

            cfg_mgr.borrow_mut ().create_config::<Config> ("backend_manager");
            cfg_mgr.borrow ().save_config ("backend_manager", &self._config).unwrap ();
        }*/
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

                    let get_backend_info: Symbol<unsafe extern fn () -> Info> = unsafe {

                        match lib.get (b"get_backend_info\0") {

                            Ok (l) => l,
                            Err (e) => {

                                warn! ("Could not find function 'get_backend_info' in library {:?}\n{}",
                                       &path.file_name ().unwrap (), e);
                                continue;
                            }
                        }
                    };

                    // Create a new instance of the plugin, and add it to the list
                    unsafe {
                        self._backend_list.push (get_backend_info ());
                    }

                    // Set the path and state of the backend plugin
                    let index = self._backend_list.len () - 1;
                    self._backend_list[index].path  = path.to_str ().unwrap ().to_string ();
                    self._backend_list[index].state = State::Unloaded;

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
    ///
    /// Only backends that are either active or unloaded will be displayed.
    pub fn list_backends (&self, backend_type: Type) -> Vec<String> {

        let mut return_vec = Vec::<String>::new ();

        for backend in &self._backend_list {

            if backend.b_type == backend_type &&
               backend.state != State::Disabled {

                return_vec.push (backend.name.clone ());
            }
        }

        return_vec
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Gets the default backend.
    pub fn get_default_backend (&self, backend_type: Type) -> Info {

        let backend_name = self._config.default_backend [backend_type as usize].clone ();

        // Check if the default backend has been set.
        // If not, pick the first suitable backend from the list.
        if backend_name == "None" {

            for item in &self._backend_list {

                if item.b_type == backend_type {
                    return item.clone ();
                }
            }
        }

        // Otherwise return the default backend for that type.
        else {

            // Find the plugin by its name and return it.
            for item in &self._backend_list {

                if item.name == backend_name {
                    return item.clone ();
                }
            }

            // If plugin name not found, get first suitable type.
            for item in &self._backend_list {

                if item.b_type == backend_type {
                    return item.clone ();
                }
            }
        }

        warn! ("No suitable backend for {:?} was found. Dropping into fallback mode.", backend_type);

        Info {name:   "Fallback".to_string (),
              author: "Fallback".to_string (),
              version: Version {major: 0, minor: 0, patch: 0},
              description: "Fallback".to_string (),
              path: "Fallback".to_string (),
              b_type: Type::Fallback,
              state: State::Unloaded}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the default backend.
    pub fn set_default_backend (&mut self, backend_name: &str) {

        for b in &self._backend_list {

            if b.name == backend_name {

                self._config.default_backend [b.b_type as usize] = b.name.clone ();
                return;
            }
        }

        warn! ("No backend plugin with name '{}' found.", backend_name);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the backend state.
    pub fn set_backend_state (&mut self, backend_name: &str, backend_state: State) {

        for b in &mut self._backend_list {

            if b.name == backend_name {
                
                b.state = backend_state;
                break;
            }
        }
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

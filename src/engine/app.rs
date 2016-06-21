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

use ::engine::config;
use ::engine::backend;
use ::util::Logger;

use std::cell::RefCell;
use std::rc::Rc;
use std::boxed::Box;
use std::process;

/*===============================================================================================*/
/*------STATIC VARIABLES-------------------------------------------------------------------------*/
/*===============================================================================================*/

static mut APP_POINTER: Option <*mut App> = None;

/*===============================================================================================*/
/*------APP STRUCT-------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The app
///
/// This is the main control center of ionCore.
/// It is in charge of initialization, updating, and shutdown of all modules,
/// as well as the handing of any inter-module communication.
#[derive (Clone)]
pub struct App {

    // Public
    /// The backend manager.
    pub backend_mgr: Rc<RefCell<backend::Manager>>,
    /// The config manager.
    pub config_mgr: Rc<RefCell<config::Manager>>,
}

/*===============================================================================================*/
/*------APPVERSION PUBLIC STATIC METHODS---------------------------------------------------------*/
/*===============================================================================================*/

impl App {

    /// Initializes the app
    pub fn init () {

        // Check if not already initialized
        if !App::is_initialized () {

            Logger::init ("./ionCore.log", true).unwrap ();
            info! ("Initializing ionCore | Version: {}", env! ("CARGO_PKG_VERSION"));

            let ab = Box::new (App {

                config_mgr: Rc::new (RefCell::new (config::Manager::new ())),
                backend_mgr: Rc::new (RefCell::new (backend::Manager::new ())),
            });

            unsafe {APP_POINTER = Some (Box::into_raw (ab))};
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Load the config files for all modules.
    pub fn load_config () {

        // Query the config directory
        let cfg_mgr = App::get_config_manager ().unwrap ();
        cfg_mgr.borrow_mut ().query_config_dir ();

        // Load config files
        let backend_mgr = App::get_backend_manager ().unwrap ();
        backend_mgr.borrow_mut ().load_config ();
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Checks if the app has been initialized.
    ///
    /// # Return value
    /// A bool returning whether the app has been initialized.
    pub fn is_initialized () -> bool {

        unsafe {

            match APP_POINTER {

                Some (pointer) => !pointer.is_null (),
                None => false
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a pointer to the backend manager instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::engine::App;
    /// # App::init ();
    /// let backend_mgr = App::get_backend_manager ().unwrap ();
    /// println! ("{}", backend_mgr.borrow ().default_audio_backend);
    pub fn get_backend_manager () -> Result<Rc<RefCell<backend::Manager>>, ()> {

        // Check if app is initialized
        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()}.backend_mgr.clone ());
        }

        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a pointer to the config manager instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::engine::App;
    /// #
    /// # App::init ();
    /// let cfg = App::get_config_manager ().unwrap ();
    /// println! ("{}", cfg.borrow ().config_dir);
    pub fn get_config_manager () -> Result<Rc<RefCell<config::Manager>>, ()> {

        // Check if app is initialized
        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()}.config_mgr.clone ());
        }

        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases all resources, and exits the application.
    pub fn exit () {

        // Check if app is initialized
        if App::is_initialized () {

            info! ("Shutting down ion Core.");

            unsafe {

                drop (Box::from_raw (APP_POINTER.unwrap ()));
                APP_POINTER = None;
            };

            // Shut down the application
            info! ("Terminating the application.");
            process::exit (0);
        }
    }
}

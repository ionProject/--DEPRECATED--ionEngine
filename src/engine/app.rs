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

use ::engine::{ConfigManager, PluginManager};
use ::util::Logger;

use std::cell::RefCell;
use std::rc::Rc;
use std::boxed::Box;

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
    /// The config manager.
    pub config_manager: Rc<RefCell<ConfigManager>>,
    /// The plugin manager.
    pub plugin_manager: Rc<RefCell<PluginManager>>,
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

            let ab = Box::new (App {config_manager: Rc::new (RefCell::new (ConfigManager::new ())),
                                    plugin_manager: Rc::new (RefCell::new (PluginManager::new ()))});

            unsafe {APP_POINTER = Some (Box::into_raw (ab))};
        }
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

    /// Returns a pointer to the config manager instance
    ///
    /// # Examples
    /// ```
    /// # use ion_core::engine::App;
    /// #
    /// # App::init ();
    /// let cfg = App::get_config_manager ().unwrap ();
    /// println! ("{}", cfg.borrow ().config_dir);
    pub fn get_config_manager () -> Option<Rc<RefCell<ConfigManager>>> {

        // Check if app is initialized
        if App::is_initialized () {
            return Some (unsafe {&*APP_POINTER.unwrap ()}.config_manager.clone ());
        }

        None
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a pointer to the plugin manager instance
    pub fn get_plugin_manager () -> Option<Rc<RefCell<PluginManager>>> {

        // Check if app is initialized
        if App::is_initialized () {
            return Some (unsafe {&*APP_POINTER.unwrap ()}.plugin_manager.clone ());
        }

        None
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases the app instance.
    ///
    /// After this point, any app commands will be ignored.
    pub fn release () {

        // Check if app is initialized
        if App::is_initialized () {

            info! ("Shutting down ion Core.");

            unsafe {

                Box::from_raw (APP_POINTER.unwrap ());
                APP_POINTER = None;
            };
        }
    }
}

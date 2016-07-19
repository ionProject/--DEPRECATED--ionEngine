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

use ::engine::AppInfo;
use ::resource::ResourceManager;
use ::window::WindowManager;
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
pub struct App {

    // Public
    /// The application info.
    pub app_info: AppInfo,
    /// The resource manager.
    pub resource_mgr: Rc<RefCell<ResourceManager>>,
    /// The window manager.
    pub window_mgr: Rc<RefCell<WindowManager>>,
}

/*===============================================================================================*/
/*------APP PUBLIC STATIC METHODS----------------------------------------------------------------*/
/*===============================================================================================*/

impl App {

    /// Initializes the app
    pub fn init (app_info: AppInfo) {

        // Check if not already initialized
        if !App::is_initialized () {

            Logger::init ("./ionCore.log", true).unwrap ();
            info! ("Initializing ionCore | Version: {}", env! ("CARGO_PKG_VERSION"));

            let ab = Box::new (App {

                app_info: app_info,
                resource_mgr: Rc::new (RefCell::new (ResourceManager::new ())),
                window_mgr: Rc::new (RefCell::new (WindowManager::new ())),
            });

            unsafe {APP_POINTER = Some (Box::into_raw (ab))};

            // Init the managers
            App::_init_managers ();
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

    /// Gets a copy of the AppInfo instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::engine::{App, AppInfo};
    /// # App::init ();
    /// let app_info = App::get_app_info ().unwrap ();
    pub fn get_app_info () -> Result<AppInfo, ()> {

        // Check if app is initialized
        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()}.app_info.clone ());
        }

        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the AppInfo instance.
    pub fn set_app_info (app_info: AppInfo) {

        // Check if app is inistalized
        if App::is_initialized () {
            unsafe {&mut *APP_POINTER.unwrap ()}.app_info = app_info;
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a pointer to the resource manager instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::engine::App;
    /// # App::init ();
    /// let resource_mgr = App::get_resource_manager ().unwrap ();
    /// println! ("{}", resource_mgr.borrow ().load_config ());
    pub fn get_resource_manager () -> Result<Rc<RefCell<ResourceManager>>, ()> {

        // Check if app is initialized
        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()}.resource_mgr.clone ());
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

/*===============================================================================================*/
/*------APP PRIVATE STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

    // Initializes the various managers.
    fn _init_managers () {

        // Get a reference to all managers.
        let resource_mgr = App::get_resource_manager ().unwrap ();

        // Init the managers
        resource_mgr.borrow_mut ().init ();
    }
}

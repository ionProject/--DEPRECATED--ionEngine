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

    // Private
    _is_in_main_loop: bool,
    _should_exit: bool,
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
                _is_in_main_loop: false,
                _should_exit: false,
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
    /// # App::init (AppInfo::new ());
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
    /// # use ion_core::engine::{App, AppInfo};
    /// # App::init (AppInfo::new ());
    /// let resource_mgr = App::get_resource_manager ().unwrap ();
    pub fn get_resource_manager () -> Result<Rc<RefCell<ResourceManager>>, ()> {

        // Check if app is initialized
        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()}.resource_mgr.clone ());
        }

        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a pointer to the window manager instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_core::engine::{App, AppInfo};
    /// # App::init (AppInfo::new ());
    /// let window_mgr = App::get_window_manager ().unwrap ();
    pub fn get_window_manager () -> Result<Rc<RefCell<WindowManager>>, ()> {

        // Check if the app is is_initialized
        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()}.window_mgr.clone ());
        }

        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// The main app loop.
    pub fn run () {

        if App::is_initialized () {

            unsafe {&mut *APP_POINTER.unwrap ()}._is_in_main_loop = true;

            loop {

                let should_exit = unsafe {&*APP_POINTER.unwrap ()}._should_exit;

                if !should_exit {

                    App::_on_pre_render ();
                    App::_on_render ();
                    App::_on_post_render ();
                }

                else {

                    unsafe {&mut *APP_POINTER.unwrap ()}._is_in_main_loop = false;
                    return;
                }
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases all resources, and exits the application.
    pub fn exit () {

        // Check if app is initialized
        if App::is_initialized () {

            // Check if in main loop
            if unsafe {&*APP_POINTER.unwrap ()}._is_in_main_loop {

                info! ("Exiting main loop.");
                unsafe {&mut *APP_POINTER.unwrap ()}._should_exit = true;
            }

            else {

                info! ("Shutting down ion Core.");

                // Release the managers
                App::_release_managers ();

                unsafe {

                    drop (Box::from_raw (APP_POINTER.unwrap ()));
                    APP_POINTER = None;
                };

                // Release the logger, and shutdown the application
                info! ("Terminating the application.");
                Logger::release ();

                process::exit (0);
            }
        }
    }

/*===============================================================================================*/
/*------APP PRIVATE STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

    // Initializes the various managers.
    fn _init_managers () {

        // Get a reference to all managers.
        let resource_mgr = App::get_resource_manager ().unwrap ();
        let window_mgr   = App::get_window_manager   ().unwrap ();

        // Init the managers
        resource_mgr.borrow_mut ().init ();
        window_mgr.borrow_mut   ().init ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // On pre render
    fn _on_pre_render () {

    }

/*-----------------------------------------------------------------------------------------------*/

    // On render
    fn _on_render () {

    }

/*-----------------------------------------------------------------------------------------------*/

    // On post render
    fn _on_post_render () {
        
    }

/*-----------------------------------------------------------------------------------------------*/

    // Releases the managers
    fn _release_managers () {

        // Get a reference to all managers.
        let window_mgr = App::get_window_manager ().unwrap ();

        // Release the managers
        window_mgr.borrow_mut ().release ();
    }
}

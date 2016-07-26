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
use ::util::{Version, Logger};

use std::cell::RefCell;
use std::rc::Rc;
use std::boxed::Box;
use std::process;

/*===============================================================================================*/
/*------STATIC VARIABLES-------------------------------------------------------------------------*/
/*===============================================================================================*/

pub static mut APP_POINTER: Option <*mut App> = None;

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
/*------APP PUBLIC METHODS-----------------------------------------------------------------------*/
/*===============================================================================================*/

impl App {

    /// The main app loop.
    pub fn run (&mut self) {

        self._is_in_main_loop = true;

        loop {

            let should_exit = self._should_exit;

            if !should_exit {

                self._on_pre_render ();
                self._on_render ();
                self._on_post_render ();
            }

            else {

                self._is_in_main_loop = false;
                return;
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases all resources, and exits the application.
    pub fn exit (&mut self) {

        // Check if in main loop
        if self._is_in_main_loop {

            info! ("Exiting main loop.");
            self._should_exit = true;
        }

        else {

            info! ("Shutting down ion Core.");

            // Release the managers, and release self
            self._release_managers ();
            App::_terminate ();
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Initializes the app
    pub fn init (&self) {

        Logger::init ("./ionCore.log", true).unwrap ();
        info! ("Initializing ionCore | Version: {}", env! ("CARGO_PKG_VERSION"));

        // Init the managers
        self._init_managers ();
    }

/*===============================================================================================*/
/*------APP PUBLIC STATIC METHODS----------------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new App Builder.
    pub fn builder () -> AppBuilder {
        AppBuilder::new ()
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

    /// Returns a reference to the app instance.
    pub fn get_instance () -> Result<&'static App, ()> {

        if App::is_initialized () {
            return Ok (unsafe {&*APP_POINTER.unwrap ()});
        }

        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a mutable reference to the app instance.
    pub fn get_instance_mut () -> Result<&'static mut App, ()> {

        if App::is_initialized () {
            return Ok (unsafe {&mut *APP_POINTER.unwrap ()});
        }

        Err (())
    }

/*===============================================================================================*/
/*------APP PRIVATE METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

    // Initializes the various managers.
    fn _init_managers (&self) {

        self.resource_mgr.borrow_mut ().init ();
        self.window_mgr.borrow_mut   ().init ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // TODO: Finish me
    // On pre render
    fn _on_pre_render (&self) {
        self.window_mgr.borrow_mut ().on_pre_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // TODO: Finish me
    // On render
    fn _on_render (&self) {
        self.window_mgr.borrow_mut ().on_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // TODO: Finish me
    // On post render
    fn _on_post_render (&self) {
        self.window_mgr.borrow_mut ().on_post_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    // Releases the managers
    fn _release_managers (&self) {
        self.window_mgr.borrow_mut ().release ();
    }

/*===============================================================================================*/
/*------APP PRIVATE STATIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

    // Release the app pointer, and terminate the application.
    fn _terminate () {

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

/*===============================================================================================*/
/*------APP BUILDER STRUCT-----------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used for building a new, static app instance.
pub struct AppBuilder {

    // Private
    _app_name: String,
    _app_developer: String,
    _app_publisher: String,
    _app_version: Version,
}

/*===============================================================================================*/
/*------APP BUILDER PUBLIC METHODS---------------------------------------------------------------*/
/*===============================================================================================*/

impl AppBuilder {

    /// Sets the app name.
    pub fn name (&mut self, name: &str) -> &mut Self {

        self._app_name = name.to_string ();
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the app developer.
    pub fn developer (&mut self, developer: &str) -> &mut Self {

        self._app_developer = developer.to_string ();
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the app publisher
    pub fn publisher (&mut self, publisher: &str) -> &mut Self {

        self._app_publisher = publisher.to_string ();
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the app version
    pub fn version (&mut self, version: Version) -> &mut Self {

        self._app_version = version;
        self
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Builds the app.
    pub fn build (&self) -> Result<&'static mut App, ()> {

        // Check if initialized
        if !App::is_initialized () {

            let app_info = AppInfo {

                app_name: self._app_name.clone (),
                app_developer: self._app_developer.clone (),
                app_publisher: self._app_publisher.clone (),
                app_version: self._app_version
            };

            let ab = Box::new (App {

                app_info: app_info,
                resource_mgr: Rc::new (RefCell::new (ResourceManager::new ())),
                window_mgr: Rc::new (RefCell::new (WindowManager::new ())),
                _is_in_main_loop: false,
                _should_exit: false,
            });

            unsafe {APP_POINTER = Some (Box::into_raw (ab))};
        }

        App::get_instance_mut ()
    }

/*===============================================================================================*/
/*------APP BUILDER PUBLIC STATIC METHODS--------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new app builder instance.
    pub fn new () -> AppBuilder {

        AppBuilder {

            _app_name: "Unknown".to_string (),
            _app_developer: "Unknown".to_string (),
            _app_publisher: "Unknown".to_string (),
            _app_version: Version::new ()
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for AppBuilder {

    fn default () -> AppBuilder {
        AppBuilder::new ()
    }
}

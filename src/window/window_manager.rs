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

use ::engine::App;
use ::window::{WindowConfig, WindowState};
use ::window::detail::{WindowBackendDefault, WindowFactoryDefault};
use ::window::traits::{WindowBackend, WindowFactory};

/*===============================================================================================*/
/*------WINDOW MANAGER STRUCT--------------------------------------------------------------------*/
/*===============================================================================================*/

/// Handles the creation and destruction of windows.
pub struct WindowManager {

    // Private
    _window_config: WindowConfig,
    _window_factory: Option<Box<WindowFactory>>,
    _window_backend: Option<Box<WindowBackend>>,
}

/*===============================================================================================*/
/*------WINDOW MANAGER PUBLIC METHODS------------------------------------------------------------*/
/*===============================================================================================*/

impl WindowManager {

    /// Initializes the window manager
    pub fn init (&mut self) {

        info! ("Initializing the Window Manager.");

        // Get a reference to the resource manager and load the window config
        let resource_mgr  = App::get_resource_manager ().unwrap ();
        let config_result = resource_mgr.borrow ().load_config::<WindowConfig> ("window");

        if let Ok (config) = config_result {
            self._window_config = config;
        }

        else {
            
            match resource_mgr.borrow ().new_config::<WindowConfig> ("window") {
                Ok (_) | Err (_) => {}
            }
        }

        // Initialize the window
        info! ("Creating a new window.");
        self._window_backend.as_mut ().unwrap ().init (&self._window_config);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Registers the window plugin.
    pub fn register_plugin (&mut self, window_factory: Box<WindowFactory>) {

        self._window_backend = Some (window_factory.get_window_backend ());
        self._window_factory = Some (window_factory);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Called on pre render.
    pub fn on_pre_render (&mut self) {

        // Check the window state
        if let WindowState::Closed = self._window_backend.as_ref ().unwrap ().get_window_state () {
            App::exit ();
        }

        self._window_backend.as_mut ().unwrap ().on_pre_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Called on render.
    pub fn on_render (&mut self) {
        self._window_backend.as_mut ().unwrap ().on_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Called on post render.
    pub fn on_post_render (&mut self) {
        self._window_backend.as_mut ().unwrap ().on_post_render ();
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases the window manager.
    pub fn release (&mut self) {

        info! ("Releasing the Window Manager.");

        self._window_backend = None;
        self._window_factory = None;
    }

/*===============================================================================================*/
/*------WINDOW MANAGER PUBLIC STATIC METHODS-----------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new instance of the Window Manager.
    pub fn new () -> WindowManager {

        WindowManager {_window_config:  WindowConfig::default (),
                       _window_factory: Some (Box::new (WindowFactoryDefault::new ())),
                       _window_backend: Some (Box::new (WindowBackendDefault::new ()))}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowManager {

    fn default () -> WindowManager {
        WindowManager::new ()
    }
}

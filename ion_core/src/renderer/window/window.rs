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
use ::renderer::traits::WindowBackend;
use ::renderer::traits::_detail::WindowBackendDefault;
use ::renderer::window::WindowConfig;
use ::util::math::Vec2;

/*===============================================================================================*/
/*------WINDOW STRUCT----------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Handles the creation and destruction of a single window.
pub struct Window {

    // Private
    _window_backend: Option<Box<WindowBackend>>,
}

/*===============================================================================================*/
/*------WINDOW PUBLIC METHODS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl Window {
    
    /// Initializes the window.
    pub fn init (&mut self, config: &WindowConfig) {

        info! ("Creating the window.");
        self._register_callbacks ();
        self._window_backend.as_mut ().unwrap ().init (config);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the window backend.
    pub fn set_backend (&mut self, backend: Box<WindowBackend>) {
        self._window_backend = Some (backend);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Processes window events.
    pub fn process_events (&mut self) {
        self._window_backend.as_mut ().unwrap ().process_events ();
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases the window.
    pub fn release (&mut self) {

        info! ("Releasing window");
        self._window_backend = None;
    }

/*===============================================================================================*/
/*------WINDOW PUBLIC STATIC METHODS-------------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new instance of the window.
    pub fn new () -> Window {

        Window {
            _window_backend: Some (Box::new (WindowBackendDefault))
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for Window {

    fn default () -> Window {
        Window::new ()
    }
}

/*===============================================================================================*/
/*------WINDOW PRIVATE METHODS-------------------------------------------------------------------*/
/*===============================================================================================*/

impl Window {

    // Registers callbacks.
    fn _register_callbacks (&mut self) {

        let window_backend = self._window_backend.as_mut ().unwrap ();

        window_backend.set_create_callback (Box::new (Window::_callback_window_create));
        window_backend.set_move_callback   (Box::new (Window::_callback_window_move));
        window_backend.set_resize_callback (Box::new (Window::_callback_window_resized));
        window_backend.set_close_callback  (Box::new (Window::_callback_window_closed));
    }

/*===============================================================================================*/
/*------WINDOW PRIVATE STATIC METHODS------------------------------------------------------------*/
/*===============================================================================================*/

    // Called on window creation.
    fn _callback_window_create () {

    }

/*-----------------------------------------------------------------------------------------------*/

    // Called on window move.
    fn _callback_window_move (_pos: Vec2) {

    }

/*-----------------------------------------------------------------------------------------------*/

    // Called on window resize.
    fn _callback_window_resized (_size: Vec2) {

    }

/*-----------------------------------------------------------------------------------------------*/

    // Called on window closed.
    fn _callback_window_closed () {
        App::get_instance_mut ().unwrap ().exit ();
    }
}

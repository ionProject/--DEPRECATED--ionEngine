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
use ::renderer::detail::WindowBackendDefault;
use ::renderer::traits::WindowBackend;
use ::renderer::window::{WindowConfig, WindowState};

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
        self._window_backend.as_mut ().unwrap ().init (config);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the window backend.
    pub fn set_backend (&mut self, backend: Box<WindowBackend>) {
        self._window_backend = Some (backend);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Called on pre render.
    pub fn on_pre_render (&mut self) {

        // Check window state
        if let WindowState::Closed = self._window_backend.as_ref ().unwrap ().get_window_state () {
            App::get_instance_mut ().unwrap ().exit ();
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

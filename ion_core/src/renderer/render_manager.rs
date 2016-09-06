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
use ::renderer::RenderConfig;
use ::renderer::traits::RenderFactory;
use ::renderer::traits::_detail::RenderFactoryDefault;
use ::renderer::window::Window;

use std::cell::RefCell;
use std::rc::Rc;

/*===============================================================================================*/
/*------RENDER MANAGER STRUCT--------------------------------------------------------------------*/
/*===============================================================================================*/

/// Manages all entities in the rendering module.
pub struct RenderManager {

    // Public
    /// Window instance.
    pub window: Rc<RefCell<Window>>,

    // Private
    _render_factory: Option<Box<RenderFactory>>,
}

/*===============================================================================================*/
/*------RENDER MANAGER PUBLIC METHODS------------------------------------------------------------*/
/*===============================================================================================*/

impl RenderManager {

    /// Initializes the renderer manager.
    pub fn init (&self) {

        info! ("Initializing the Render Manager.");

        // Get a reference to the resource manager and load the renderer config
        let mut render_config = RenderConfig::new ();
        let resource_mgr  = App::get_instance ().unwrap ().resource_mgr.clone ();
        let config_result = resource_mgr.borrow ().load_config::<RenderConfig> ("renderer");

        if let Ok (config) = config_result {
            render_config = config;
        }

        else {

            match resource_mgr.borrow ().new_config::<RenderConfig> ("renderer") {
                Ok (_) | Err (_) => {}
            }
        }

        // Initialize the window
        self.window.borrow_mut ().init (&render_config.window_config);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Registers the renderer plugin.
    pub fn register_plugin (&mut self, render_factory: Box<RenderFactory>) {

        self.window.borrow_mut ().set_backend (render_factory.get_window_backend ());
        self._render_factory = Some (render_factory);
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Processes the window events.
    pub fn process_window_events (&mut self) {
        self.window.borrow_mut ().process_events ();
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Releases the Render Manager.
    pub fn release (&mut self) {

        info! ("Releasing the Render Manager");

        self.window.borrow_mut ().release ();
        self._render_factory = None;
    }

/*===============================================================================================*/
/*------RENDER MANAGER PUBLIC STATIC METHODS-----------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a new instance of the Render Manager.
    pub fn new () -> RenderManager {

        RenderManager {

            window: Rc::new (RefCell::new (Window::new ())),
            _render_factory: Some (Box::new (RenderFactoryDefault)),
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for RenderManager {

    fn default () -> RenderManager {
        RenderManager::new ()
    }
}

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

use ::renderer::window::{WindowConfig, WindowState};
use ::renderer::traits::WindowBackend;
use ::util::traits::AsAny;

use std::any::Any;
use std::os::raw::c_void;
use std::ptr::null_mut;

/*===============================================================================================*/
/*------WINDOW BACKEND DEFAULT STRUCT------------------------------------------------------------*/
/*===============================================================================================*/

/// Used as the default window backend.
///
/// This struct is not a functioning backend. It is only used in situations where
/// a window plugin is either not specified, or fails to load.
pub struct WindowBackendDefault;

/*===============================================================================================*/
/*------WINDOW BACKEND DEFAULT PUBLIC METHODS----------------------------------------------------*/
/*===============================================================================================*/

impl WindowBackend for WindowBackendDefault {

    fn init (&mut self, _: &WindowConfig) {

        warn! ("The default window backend is currently being used.\n\
                While the application will continue running, it may not behave as expected.");
    }

/*-----------------------------------------------------------------------------------------------*/

    fn get_window_state (&self) -> WindowState {
        WindowState::Active
    }

/*-----------------------------------------------------------------------------------------------*/

    fn on_pre_render  (&mut self) {}
    fn on_render      (&mut self) {}
    fn on_post_render (&mut self) {}

/*-----------------------------------------------------------------------------------------------*/

    unsafe fn get_platform_display (&self) -> Result<*mut c_void, ()> {
        Ok (null_mut ())
    }

/*-----------------------------------------------------------------------------------------------*/

    unsafe fn get_platform_window (&self) -> Result<*mut c_void, ()> {
        Ok (null_mut ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AsAny for WindowBackendDefault {

    fn as_any (&self) -> &Any {
        self
    }
}

/*===============================================================================================*/
/*------WINDOW BACKEND DEFAULT PUBLIC STATIC METHODS---------------------------------------------*/
/*===============================================================================================*/

impl WindowBackendDefault {

    pub fn new () -> WindowBackendDefault {
        WindowBackendDefault
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowBackendDefault {

    fn default () -> WindowBackendDefault {
        WindowBackendDefault::new ()
    }
}

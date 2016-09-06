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

use ::renderer::window::WindowConfig;
use ::util::traits::AsAny;
use ::util::math::Vec2;

use std::os::raw::c_void;

/*===============================================================================================*/
/*------WINDOW BACKEND TRAIT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used for backend agnostic window creation.
///
/// Window backend plugins implement this trait. The backend is then accessed by the
/// Window Manager via a `get_window` function.
pub trait WindowBackend: AsAny {

    /// Initializes the window.
    fn init (&mut self, _config: &WindowConfig) {

        warn! ("The default window backend is currently being used.\n\
                While the application will continue running, it may not behave as expected.");
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the creation callback.
    fn set_create_callback (&mut self, _callback: Box<Fn ()>) {

    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the move callback.
    fn set_move_callback (&mut self, _callback: Box<Fn (Vec2)>) {

    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the resize callback.
    fn set_resize_callback (&mut self, _callback: Box<Fn (Vec2)>) {

    }

/*-----------------------------------------------------------------------------------------------*/

    /// Sets the close callback.
    fn set_close_callback (&mut self, _callback: Box<Fn ()>) {

    }

/*-----------------------------------------------------------------------------------------------*/

    /// Processes window events.
    fn process_events (&mut self) {

    }

/*-----------------------------------------------------------------------------------------------*/

    /// Get a raw pointer to the platform display.
    unsafe fn get_platform_display (&self) -> Result<*mut c_void, ()> {
        Err (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Get a raw pointer to the platform window.
    unsafe fn get_platform_window  (&self) -> Result<*mut c_void, ()> {
        Err (())
    }
}

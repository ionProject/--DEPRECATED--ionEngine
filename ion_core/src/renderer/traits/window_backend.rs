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
use ::util::traits::AsAny;

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
    fn init (&mut self, config: &WindowConfig);
    /// Gets the current window state.
    fn get_window_state (&self) -> WindowState;
    /// On pre render event.
    fn on_pre_render (&mut self);
    /// On render event.
    fn on_render (&mut self);
    /// On post render even.
    fn on_post_render (&mut self);

    /// Get a raw pointer to the platform display.
    unsafe fn get_platform_display (&self) -> Result<*mut c_void, ()>;
    /// Get a raw pointer to the platform window.
    unsafe fn get_platform_window  (&self) -> Result<*mut c_void, ()>;
}

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

use ::window::WindowConfig;

/*===============================================================================================*/
/*------WINDOW MANAGER STRUCT--------------------------------------------------------------------*/
/*===============================================================================================*/

/// Handles the creation and destruction of windows.
#[derive (Copy, Clone)]
pub struct WindowManager {

    // Private
    _window_config: WindowConfig,
}

/*===============================================================================================*/
/*------WINDOW MANAGER PUBLIC STATIC METHODS-----------------------------------------------------*/
/*===============================================================================================*/

impl WindowManager {

    /// Returns a new instance of the Window Manager.
    pub fn new () -> WindowManager {
        WindowManager {_window_config: WindowConfig::default ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowManager {

    fn default () -> WindowManager {
        WindowManager::new ()
    }
}

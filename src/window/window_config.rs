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

use ::util::math::Vec2;

/*===============================================================================================*/
/*------WINDOW CONFIG STRUCT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores the config for the window manager.
#[derive (Deserialize, Serialize)]
pub struct WindowConfig {

    // Public
    /// The window title.
    pub window_title: String,
    /// The window size.
    pub window_size: Vec2,
    /// The window position.
    pub window_pos: Vec2,
}

/*===============================================================================================*/
/*------WINDOW CONFIG PUBLIC STATIC METHODS------------------------------------------------------*/
/*===============================================================================================*/

impl WindowConfig {

    /// Returns a new Window Config instance.
    pub fn new () -> WindowConfig {

        WindowConfig {window_title: "Untitled Window".to_string (),
                      window_size: Vec2 {x: 1024.0, y: 768.0},
                      window_pos: Vec2::new ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowConfig {

    fn default () -> WindowConfig {
        WindowConfig::new ()
    }
}

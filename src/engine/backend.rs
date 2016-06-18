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

extern crate serde;

use self::serde::{Serialize, Deserialize};

/*===============================================================================================*/
/*------BACKENDCONFIG STRUCT---------------------------------------------------------------------*/
/*===============================================================================================*/

#[derive (Serialize, Deserialize)]
pub struct BackendConfig {

    // Public
    /// The backend directory.
    pub backend_dir: String,
    /// The default window backend.
    pub default_window_backend: String,
    /// The default rendering backend.
    pub default_rendering_backend: String,
    /// The default audio backend.
    pub default_audio_backend: String,
}

/*===============================================================================================*/
/*------BACKENDCONFIG PUBLIC STATIC METHODS------------------------------------------------------*/
/*===============================================================================================*/

impl Default for BackendConfig {

    fn default () -> BackendConfig {

        BackendConfig {

            backend_dir: "backend".to_string (),
            default_window_backend: "fallback".to_string (),
            default_rendering_backend: "fallback".to_string (),
            default_audio_backend: "fallback".to_string ()
        }
    }
}

/*===============================================================================================*/
/*------BACKENDMANAGER STRUCT--------------------------------------------------------------------*/
/*===============================================================================================*/

/// Manages the loading of the various backend systems.
#[derive (Default)]
pub struct BackendManager {

    // Public
    /// The backend configuration.
    pub backend_config: BackendConfig
}

/*===============================================================================================*/
/*------BACKENDMANAGER PUBLIC STATIC METHODS-----------------------------------------------------*/
/*===============================================================================================*/

impl BackendManager {

    /// Creates a new instance of the BackendManager.
    ///
    /// # Return value
    /// A new instance of the backend manager.
    pub fn new () -> BackendManager {

        BackendManager {

            backend_config: BackendConfig::default ()
        }
    }
}

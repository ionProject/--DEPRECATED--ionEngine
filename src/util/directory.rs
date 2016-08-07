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

use std::env;

/*===============================================================================================*/
/*------DIRECTORY STRUCT-------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The Directory struct.
///
/// Gives access to various engine directories.
#[derive (Copy, Clone)]
pub struct Directory;

/*===============================================================================================*/
/*------DIRECTORY PUBLIC METHODS-----------------------------------------------------------------*/
/*===============================================================================================*/

impl Directory {

    /// Returns the resource directory.
    pub fn get_resource_directory () -> String {
        "res/".to_string ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the binary directory.
    pub fn get_binary_directory () -> String {
        "bin/".to_string ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the plugin directory.
    pub fn get_plugin_directory () -> String {
        "bin/plugins/".to_string ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the config directory.
    pub fn get_config_directory () -> String {
        "config/".to_string ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the persistent data directory.
    ///
    /// This is where the engine has read/write permissions, and the data persists between runs.
    pub fn get_persistent_data_directory () -> String {

        let app = App::get_instance ().unwrap ();
        format! ("{}/.{}/{}/", env::home_dir ().unwrap ().display (), &app.project_developer, &app.project_name)
    }
    
/*-----------------------------------------------------------------------------------------------*/

    /// Returns the persistent config directory.
    ///
    /// This is a folder in the persistent data directory where config files are stored.
    pub fn get_persistent_config_directory () -> String {

        let app = App::get_instance ().unwrap ();
        format! ("{}/.{}/{}/config/", env::home_dir ().unwrap ().display (), &app.project_developer, &app.project_name)
    }
}

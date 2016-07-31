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

use ::util::Version;

/*===============================================================================================*/
/*------PROJECT CONFIG STRUCT--------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used for the loading and saving of the project configuration.
#[derive (Deserialize, Serialize)]
pub struct ProjectConfig {

    // Public
    /// The project name.
    pub project_name: String,
    /// The project developer.
    pub project_developer: String,
    /// The project version.
    pub project_version: Version,
}

/*===============================================================================================*/
/*------PROJECT CONFIG PUBLIC STATIC METHODS-----------------------------------------------------*/
/*===============================================================================================*/

impl ProjectConfig {

    /// Returns a new instance of the Project Config.
    pub fn new () -> ProjectConfig {

        ProjectConfig {

            project_name:      "Untitled".to_string (),
            project_developer: "Unknown".to_string (),
            project_version:   Version::new (),
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for ProjectConfig {

    fn default () -> ProjectConfig {
        ProjectConfig::new ()
    }
}

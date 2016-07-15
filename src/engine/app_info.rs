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
/*------APP INFO STRUCT--------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores the application information.
///
/// This just includes basic information such as the name of the application,  
/// the developer, the publisher, and the version.
#[derive (Clone)]
pub struct AppInfo {

    // Public
    /// The application name.
    pub app_name: String,
    /// The application author.
    pub app_developer: String,
    /// The application publisher.
    pub app_publisher: String,
    /// The application version.
    pub app_version: Version,
}

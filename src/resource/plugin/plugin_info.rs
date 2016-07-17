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
/*------PLUGIN INFO STRUCT-----------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores basic information on a plugin.
#[derive (Debug)]
pub struct PluginInfo {

    // Public
    /// The plugin name.
    plug_name: String,
    /// The plugin developer.
    plug_developer: String,
    /// The plugin description.
    plug_description: String,
    /// The plugin version.
    plug_version: Version,
}

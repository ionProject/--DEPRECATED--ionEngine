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

use ::resource::ResourceDirectory;

use std::vec::Vec;

/*===============================================================================================*/
/*------RESOURCE CONFIG STRUCT-------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores the configuration for the Resource Manager.
#[derive (Clone, Debug, Serialize, Deserialize)]
pub struct ResourceConfig {

    // Public
    /// A list of resource directories.
    resource_dir_list: Vec<ResourceDirectory>,
}

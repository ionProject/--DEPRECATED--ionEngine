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

/*===============================================================================================*/
//! The resource module.
//!
//! Everything related to resource loading, management, and unloading is handled here.  
//! There are two main parts to it:
//!
//! 1. The Resource Manager.
//! 2. Resource Loaders.
//!
//! The Resource Manager is the main interface to the resource module.  
//! (Almost) all interaction with the module goes through it.
//!
//! The loaders are small structs that take care of a single resource (eg. the Texture  
//! Loader would handle textures, the Plugin Loader would handle plugins, etc).  
//! Each loader is indirectly accessed via the Resource Manager.
/*===============================================================================================*/

// Modules
mod config;
mod resource_manager;

pub use self::resource_manager::ResourceManager;

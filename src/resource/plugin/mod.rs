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
//! The plugin module.
//!
//! Contains the plugin loader.
/*===============================================================================================*/

// Modules
mod plugin_config;
mod plugin_loader;
mod plugin_info;

pub use self::plugin_config::PluginConfig;
pub use self::plugin_loader::PluginLoader;
pub use self::plugin_info::PluginInfo;

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
//! The backend module.
//!
//! Contains everything required to load and manage different backend systems.
/*===============================================================================================*/

mod backend_config;
mod backend_enum;
mod backend_manager;
mod backend_plugin;

pub use self::backend_config::Config;
pub use self::backend_enum::Type;
pub use self::backend_manager::Manager;
pub use self::backend_plugin::Plugin;

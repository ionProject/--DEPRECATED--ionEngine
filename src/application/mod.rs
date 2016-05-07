/*================================================================================================*/
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
/*================================================================================================*/

/*================================================================================================*/
//! The application module
//!
//! This module is the control center of ionCore.
//! It handles the initialization, updating, and destruction of all other modules,
//! as well as any inter-module communication.
/*================================================================================================*/

// Static variables
/// The major version of the application
pub static APP_VERSION_MAJOR : &'static str = env! ("CARGO_PKG_VERSION_MAJOR");
/// The minor version of the application
pub static APP_VERSION_MINOR : &'static str = env! ("CARGO_PKG_VERSION_MINOR");
/// The patch version of the application
pub static APP_VERSION_PATCH : &'static str = env! ("CARGO_PKG_VERSION_PATCH");

// Modules
mod app;
mod app_builder;

pub use self::app::App;

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
/*------BACKEND TYPE ENUM------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The backend type.
///
/// Used to determine the type of a backend plugin.
#[derive (Debug, Copy, Clone, PartialEq)]
pub enum Type {

    /// Audio type.
    Audio,
    /// Renderer type.
    Renderer,
    /// Window type.
    Window,
}

/*===============================================================================================*/
/*------BACKEND STATE ENUM-----------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores the state of the backend plugins.
#[derive (Debug, Copy, Clone)]
pub enum State {

    /// The plugin is not loaded.
    /// This is the default.
    Unloaded,
    /// The plugin is active.
    /// This means the plugin has been loaded, and is being used.
    Active,
    /// The plugin is disabled and cannot be used.
    /// This usually means the plugin has failed to load.
    Disabled,
}

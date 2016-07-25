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
/*------WINDOW STATE ENUM------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores the current window state.
#[derive (Copy, Clone)]
pub enum WindowState {

    /// The default window state.
    Active,
    /// Window is open, but not currently selected.
    Inactive,
    /// Window is minimized.
    Minimized,
    /// Window is maximized.
    Maximized,
    /// Window has been closed.
    Closed
}

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
/*------WINDOW BACKEND TRAIT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used for backend agnostic window creation.
///
/// Window backend plugins implement this trait. The backend is then accessed by the
/// Window Manager via a `get_window` function.
pub trait WindowBackend {

    /// Initializes the window.
    fn init (&self);
    /// Sets the title of the window.
    fn get_title (&self) -> String;
    /// Gets the title of the window.
    fn set_title (&mut self, title: &str);
}

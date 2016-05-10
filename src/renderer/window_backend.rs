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

use ion_utils::math::Vec2;

/*================================================================================================*/
/*------WINDOWBACKEND TRAIT-----------------------------------------------------------------------*/
/*================================================================================================*/

/// Implemented by various window backends
pub trait WindowBackend {

    /// Sets the position of the window.
    ///
    /// # Arguments
    /// * `pos` - The position variable. It contains the coordinates the window position
    ///           will be set to.
    fn set_position (&mut self, pos: &Vec2);

    /// Gets the position of the window.
    ///
    /// # Return value
    /// The return value is a Vec2 containing the current position of the window.
    fn get_position (&self) -> Vec2;

    /// Sets the size of the window.
    ///
    /// # Arguments
    /// * `size` - Contains the width and height you would like to set the window to.
    fn set_size (&mut self, size: &Vec2);

    /// Gets the size of the window.
    ///
    /// # Return value
    /// The return value is a Vec2 containing the current size of the window.
    fn get_size (&self) -> Vec2;

    /// Sets the title of the window.
    ///
    /// # Arguments
    /// * `title` - A string literal which is used to set the window title
    fn set_title (&mut self, title: &str);

    /// Gets the title of the window.
    ///
    /// # Return value
    /// The return value is a &str containing the current title of the window.
    fn get_title (&self) -> &str;
}

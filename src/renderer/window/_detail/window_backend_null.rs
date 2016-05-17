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

use ::util::math::Vec2;
use ::renderer::window::WindowBackend;

/*================================================================================================*/
/*------WINDOWBACKENDNULL STRUCT------------------------------------------------------------------*/
/*================================================================================================*/

/// Window backend fallback.
///
/// In the occasion that a window cannot be loaded, the null backend is used.
/// This ensures that the app can continue without crashing.
pub struct WindowBackendNull {

    // Private
    _position: Vec2,
    _size: Vec2,
    _title: String
}

/*================================================================================================*/
/*------WINDOWBACKENDNULL PUBLIC METHODS----------------------------------------------------------*/
/*================================================================================================*/

impl WindowBackend for WindowBackendNull {

    fn set_position (&mut self, pos: &Vec2) {
        self._position = *pos;
    }

    fn get_position (&self) -> Vec2 {
        self._position
    }

    fn set_size (&mut self, size: &Vec2) {
        self._size = *size;
    }

    fn get_size (&self) -> Vec2 {
        self._size
    }

    fn set_title (&mut self, title: String) {
        self._title = title;
    }

    fn get_title (&self) -> String {
        self._title.clone ()
    }
}

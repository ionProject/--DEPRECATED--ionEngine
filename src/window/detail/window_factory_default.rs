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

use ::window::detail::WindowBackendDefault;
use ::window::traits::WindowBackend;
use ::window::traits::WindowFactory;

/*===============================================================================================*/
/*------WINDOW FACTORY STRUCT--------------------------------------------------------------------*/
/*===============================================================================================*/

/// Used as the default window factory.
///
/// This struct is not a functioning factory. It is only used in situations where
/// a window plugin is either not specified, or fails to load.
pub struct WindowFactoryDefault;

/*===============================================================================================*/
/*------WINDOW FACTORY PUBLIC METHODS------------------------------------------------------------*/
/*===============================================================================================*/

impl WindowFactory for WindowFactoryDefault {

    fn get_window_backend (&self) -> Box<WindowBackend> {
        Box::new (WindowBackendDefault::new ())
    }
}

/*===============================================================================================*/
/*------WINDOW FACTORY PUBLIC STATIC METHODS-----------------------------------------------------*/
/*===============================================================================================*/

impl WindowFactoryDefault {

    pub fn new () -> WindowFactoryDefault {
        WindowFactoryDefault {}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowFactoryDefault {

    fn default () -> WindowFactoryDefault {
        WindowFactoryDefault::new ()
    }
}

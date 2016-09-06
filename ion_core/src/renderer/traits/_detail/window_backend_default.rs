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

use ::renderer::traits::WindowBackend;
use ::util::traits::AsAny;

use std::any::Any;

/*===============================================================================================*/
/*------WINDOW BACKEND DEFAULT STRUCT------------------------------------------------------------*/
/*===============================================================================================*/

/// Used as the default window backend.
///
/// This struct is not a functioning backend. It is only used in situations where
/// a window plugin is either not specified, or fails to load.
#[derive (Copy, Clone)]
pub struct WindowBackendDefault;

/*===============================================================================================*/
/*------WINDOW BACKEND DEFAULT PUBLIC METHODS----------------------------------------------------*/
/*===============================================================================================*/

impl WindowBackend for WindowBackendDefault {

}

/*-----------------------------------------------------------------------------------------------*/

impl AsAny for WindowBackendDefault {

    fn as_any (&self) -> &Any {
        self
    }
}

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

extern crate ion_core;

use ::window::WindowBackendSDL2;

use self::ion_core::renderer::traits::{WindowBackend, RenderFactory};

/*===============================================================================================*/
/*------RENDER FACTORY STRUCT--------------------------------------------------------------------*/
/*===============================================================================================*/

/// The render factory.
#[derive (Copy, Clone)]
pub struct RenderFactorySDLGL;

/*===============================================================================================*/
/*------RENDER FACTORY STRUCT PUBLIC METHODS-----------------------------------------------------*/
/*===============================================================================================*/

impl RenderFactory for RenderFactorySDLGL {

    fn get_window_backend (&self) -> Box<WindowBackend> {
        Box::new (WindowBackendSDL2::new ())
    }
}

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
//! The traits module.
//!
//! Stores any required traits for the rendering module.
/*===============================================================================================*/

mod render_factory;
mod window_backend;

pub use self::render_factory::RenderFactory;
pub use self::window_backend::WindowBackend;

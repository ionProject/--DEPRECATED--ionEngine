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

use std::any::Any;

/*===============================================================================================*/
/*------RENDER FACTORY TRAIT---------------------------------------------------------------------*/
/*===============================================================================================*/

/// This trait is implemented by any struct which needs to be able to be cast to a contrete type.
///
/// This is useful in situations where trait objects are being stored,
/// and require downcasting back to it's original type.
pub trait AsAny {

    /// Returns an instance of `&Any`.
    fn as_any (&self) -> &Any;
}

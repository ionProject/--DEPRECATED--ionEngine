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

extern crate serde;
extern crate serde_json;

use self::serde::Deserialize;

use std::fs::File;
use std::io;
use std::io::Read;

/*===============================================================================================*/
/*------DESERIALIZER STRUCT----------------------------------------------------------------------*/
/*===============================================================================================*/

/// Providies utilities for deserializing JSON files.
#[derive (Copy, Clone)]
pub struct Deserializer;

/*===============================================================================================*/
/*------DESERIALIZER PUBLIC STATIC METHODS-------------------------------------------------------*/
/*===============================================================================================*/

impl Deserializer {

    /// Deserializes an object from a file.
    pub fn deserialize_from_file<T: Deserialize> (file_path: &str) -> Result<T, io::Error> {

        let mut file   = try! (File::open (file_path));
        let mut string = String::new ();
        try! (file.read_to_string (&mut string));

        match Deserializer::deserialize_from_string (&string) {

            Ok  (item) => Ok (item),
            Err (e) => Err (io::Error::new (io::ErrorKind::Other, e.to_string ()))
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Deserializes an object from a String.
    pub fn deserialize_from_string<T: Deserialize> (string: &str) -> Result<T, serde_json::Error> {
        
        Ok (try! (serde_json::from_str (string)))
    }
}

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

use self::serde::Serialize;

use std::fs::File;
use std::io;
use std::io::Write;

/*===============================================================================================*/
/*------SERIALIZER STRUCT------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Providies utilities for serializing JSON files.
#[derive (Copy, Clone)]
pub struct Serializer;

/*===============================================================================================*/
/*------SERIALIZER PUBLIC STATIC METHODS---------------------------------------------------------*/
/*===============================================================================================*/

impl Serializer {

    /// Serializes an object to a file.
    ///
    /// The contents of type `T` is converted into a JSON text file.  
    /// `T` must derive from the [`serde::Serialize`](http://serde-rs.github.io/serde/serde/ser/trait.Serialize.html)
    /// trait.
    ///
    /// # Arguments
    /// * `item` - The instance of `T` to serialize.
    /// * `file_path` - The path to the file to be serialized.  
    /// It can be either a full or local path.  
    /// If the file doesn't exist, it will be created.
    ///
    /// # Returns
    /// A `Result` containing a `std::io::Error` on failure.
    ///
    /// # Examples
    /// ```
    /// # #![feature (custom_derive)]
    /// # #![feature (plugin)]
    /// # #![plugin (serde_macros)]
    /// # extern crate serde;
    /// # extern crate ion_core;
    /// # use self::ion_core::util::serialization::Serializer;
    /// # use self::serde::Serialize;
    /// #[derive (Serialize)]
    /// struct Person {
    ///     
    ///     name: String,
    ///     age: u32
    /// }
    /// # fn main () {
    /// let person = Person {name: "John Doe".to_string (), age: 31};
    /// Serializer::to_file (&person, "person.json");
    /// # }
    /// ```
    pub fn to_file<T: Serialize> (item: &T, file_path: &str) -> Result<(), io::Error> {

        let mut file = try! (File::create (file_path));

        let string = match Serializer::to_string (item) {

            Ok  (s) => s,
            Err (e) => return Err (io::Error::new (io::ErrorKind::Other, e.to_string ()))
        };

        try! (file.write (string.as_bytes ()));
        Ok (())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Serializes an object to a String.
    ///
    /// The type `T` is converted into a JSON string.
    /// `T` must derive from the [`serde::Serialize`](http://serde-rs.github.io/serde/serde/ser/trait.Serialize.html)
    /// trait.
    ///
    /// # Arguments
    /// * `item` - The instance of `T` to serialize.  
    ///
    /// # Returns
    /// A `Result` containing a JSON string on success,    
    /// and a `serde_json::Error` on failure.
    ///
    /// # Examples
    /// ```
    /// # #![feature (custom_derive)]
    /// # #![feature (plugin)]
    /// # #![plugin (serde_macros)]
    /// # extern crate serde;
    /// # extern crate ion_core;
    /// # use self::ion_core::util::serialization::Serializer;
    /// # use self::serde::Serialize;
    /// #[derive (Serialize)]
    /// struct Person {
    ///     
    ///     name: String,
    ///     age: u32
    /// }
    /// # fn main () {
    /// let person = Person {name: "John Doe".to_string (), age: 31};
    /// let string = Serializer::to_string (&person).unwrap ();
    /// # }
    /// ```
    pub fn to_string<T: Serialize> (item: &T) -> Result<String, serde_json::Error> {

        Ok (try! (serde_json::to_string_pretty (item)))
    }
}

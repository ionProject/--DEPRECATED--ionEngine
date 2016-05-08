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

/*================================================================================================*/
/*------APPVERSION STRUCT-------------------------------------------------------------------------*/
/*================================================================================================*/

/// The app version
///
/// This is a simple struct that stores the application version.
/// It is either set automatically (using cargo environment variables), via an app config file,
/// or manually using the AppBuilder.
///
/// Once set, it cannot be changed.
#[derive (Copy, Clone)]
pub struct AppVersion {

    // Public
    /// The major version of the application
    pub major : i32,
    /// The minor version of the application
    pub minor : i32,
    /// The patch version of the application
    pub patch : i32
}

/*================================================================================================*/
/*------APPVERSION PUBLIC METHODS-----------------------------------------------------------------*/
/*================================================================================================*/

impl AppVersion {

    /// Formats the version as a string
    pub fn to_string (&self) -> String {
        format! ("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/*================================================================================================*/
/*------APP STRUCT--------------------------------------------------------------------------------*/
/*================================================================================================*/

/// The app
///
/// This is the main control center of ionCore.
/// It is in charge of initialization, updating, and shutdown of all modules,
/// as well as the handing of any inter-module communication.
#[derive (Clone)]
pub struct App <'a> {

    // Private
    _name    : &'a str,
    _version : AppVersion
}

/*================================================================================================*/
/*------APPVERSION PUBLIC METHODS-----------------------------------------------------------------*/
/*================================================================================================*/

impl <'a> App <'a> {

    /// Returns the name of the application
    pub fn get_name (&self) -> String {
        self._name.to_owned ()
    }

    /// Returns the version of the application
    pub fn get_version (&self) -> AppVersion {
        self._version
    }
}

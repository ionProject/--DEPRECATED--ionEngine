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
//! The application module
//!
//! This module is the control center of ionCore.
//! It handles the initialization, updating, and destruction of all other modules,
//! as well as any inter-module communication.
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
pub struct App {

    // Private
    _name    : String,
    _version : AppVersion
}

/*================================================================================================*/
/*------APPVERSION PUBLIC METHODS-----------------------------------------------------------------*/
/*================================================================================================*/

impl App {

    /// Returns the name of the application
    pub fn get_name (&self) -> String {
        self._name.clone ()
    }

    /// Returns the version of the application
    pub fn get_version (&self) -> AppVersion {
        self._version
    }
}

/*================================================================================================*/
/*------APPBUILDER STRUCT-------------------------------------------------------------------------*/
/*================================================================================================*/

/// The app builder
///
/// The app builder provides an easy and straightforward way of creating and
/// configuring a new app instance.
#[derive (Clone)]
pub struct AppBuilder {

    // Private
    _name: String,
    _version: AppVersion
}

/*================================================================================================*/
/*------APPBUILDER PUBLIC METHODS-----------------------------------------------------------------*/
/*================================================================================================*/

impl AppBuilder {

    /// Builds the application
    pub fn build (&self) -> App {

        App {_name: self._name.clone (),
             _version: self._version}
    }

    /// Sets the application name
    pub fn name (&mut self, name: String) -> &mut AppBuilder {

        self._name = name;
        self
    }

    /// Sets the version
    pub fn version (&mut self, major: i32, minor: i32, patch: i32) -> &mut AppBuilder {

        self._version.major = major;
        self._version.minor = minor;
        self._version.patch = patch;

        self
    }

    /// Sets the major version
    pub fn version_major (&mut self, major: i32) -> &mut AppBuilder {

        self._version.major = major;
        self
    }

    /// Sets the minor version
    pub fn version_minor (&mut self, minor: i32) -> &mut AppBuilder {

        self._version.minor = minor;
        self
    }

    /// Sets the patch version
    pub fn version_patch (&mut self, patch: i32) -> &mut AppBuilder {

        self._version.patch = patch;
        self
    }

/*================================================================================================*/
/*------APPBUILDER PUBLIC STATIC METHODS----------------------------------------------------------*/
/*================================================================================================*/

    /// Creates a new instance of the app builder
    pub fn new () -> AppBuilder {

        AppBuilder {_name: "Ion App".to_owned (),
                    _version: AppVersion {major: i32::from_str_radix (env! ("CARGO_PKG_VERSION_MAJOR"), 10).unwrap (),
                                          minor: i32::from_str_radix (env! ("CARGO_PKG_VERSION_MINOR"), 10).unwrap (),
                                          patch: i32::from_str_radix (env! ("CARGO_PKG_VERSION_PATCH"), 10).unwrap ()}}
    }
}

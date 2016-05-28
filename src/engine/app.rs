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

use ::util::{Logger, Version};
use ::engine::PluginManager;

use std::cell::RefCell;
use std::rc::Rc;

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

    // Public
    /// Manages all plugins.
    pub plugin_manager: PluginManager,

    // Private
    ///
    pub _name: String,
    ///
    pub _developer: String,
    ///
    pub _version: Version,
    ///
    pub _self_ref: Option<Rc<RefCell<App>>>
}

/*================================================================================================*/
/*------APPVERSION PUBLIC METHODS-----------------------------------------------------------------*/
/*================================================================================================*/

impl App {

    /// Returns the name of the application.
    ///
    /// # Return value
    /// An immutable reference to the app name.
    pub fn get_name (&self) -> &str {
        &self._name
    }

    /// Returns the application developer
    ///
    /// # Return value
    /// An immutable reference to the app developer
    pub fn get_developer (&self) -> &str {
        &self._developer
    }

    /// Returns the version of the application.
    ///
    /// # Return value
    /// An immutable reference to the app version.
    pub fn get_version (&self) -> &Version {
        &self._version
    }

/*================================================================================================*/
/*------APPVERSION PUBLIC STATIC METHODS----------------------------------------------------------*/
/*================================================================================================*/

    /// Returns a new instance of the app builder
    ///
    /// # Return value
    /// A new instance of the app builder.
    pub fn builder () -> AppBuilder {

        AppBuilder {_name: "Ion App".to_string (),
                    _developer: String::new (),
                    _version: Version {major: 0, minor: 1, patch: 0}}
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
    _developer: String,
    _version: Version
}

/*================================================================================================*/
/*------APPBUILDER PUBLIC METHODS-----------------------------------------------------------------*/
/*================================================================================================*/

impl AppBuilder {

    /// Builds the application
    ///
    /// # Return value
    /// A new app instance.
    pub fn build (&self) -> App {

        Logger::init ("./ionCore.log", true).unwrap ();
        info! ("Initializing ionCore | Version: {}", env! ("CARGO_PKG_VERSION"));

        App {plugin_manager: PluginManager::new (),
             _name: self._name.clone (),
             _developer: self._developer.clone (),
             _version: self._version,
             _self_ref: None}
    }

    /// Sets the application name.
    ///
    /// # Arguments
    /// * `name` - The desired name of the application.
    ///
    /// # Return value
    /// A mutable reference to the current AppBuilder instance.
    pub fn name (&mut self, name: &str) -> &mut AppBuilder {

        self._name = name.to_string ();
        self
    }

    /// Sets the application developer.
    ///
    /// # Arguments
    /// * `developer` - The name of the developer.
    ///
    /// # Return value
    /// A mutable reference to the current AppBuilder instance.
    pub fn developer (&mut self, developer: &str) -> &mut AppBuilder {

        self._developer = developer.to_string ();
        self
    }

    /// Sets the version of the app.
    ///
    /// # Arguments
    /// * 'major' - The major version of the app.
    /// * `minor` - The minor version of the app.
    /// * `patch` - The patch version of the app.
    ///
    /// # Return value
    /// A mutable reference to the current AppBuilder instance.
    pub fn version (&mut self, major: i32, minor: i32, patch: i32) -> &mut AppBuilder {

        self._version.major = major;
        self._version.minor = minor;
        self._version.patch = patch;

        self
    }

    /// Sets the major version of the app.
    ///
    /// # Arguments
    /// * `major` - The major version of the app.
    ///
    /// # Return value
    /// A mutable reference to the current AppBuilder instance.
    pub fn version_major (&mut self, major: i32) -> &mut AppBuilder {

        self._version.major = major;
        self
    }

    /// Sets the minor version of the app.
    ///
    /// # Arguments
    /// * `minor` - The minor version of the app.
    ///
    /// # Return value
    /// A mutable reference to the current AppBuilder instance.
    pub fn version_minor (&mut self, minor: i32) -> &mut AppBuilder {

        self._version.minor = minor;
        self
    }

    /// Sets the patch version of the app.
    ///
    /// # Arguments
    /// * `patch` - The patch version of the app.
    ///
    /// # Return value
    /// A mutable reference to the current AppBuilder instance.
    pub fn version_patch (&mut self, patch: i32) -> &mut AppBuilder {

        self._version.patch = patch;
        self
    }
}

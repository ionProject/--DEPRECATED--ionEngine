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
//! The logging module.
//!
//! It provides basic logging functionality.
/*================================================================================================*/

extern crate log;

use std::boxed::Box;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::Write;

/*================================================================================================*/
/*------LOGGER STRUCT-----------------------------------------------------------------------------*/
/*================================================================================================*/

/// The logger struct
///
/// It provides basic logging capabilities, and is designed for use within ion.
pub struct Logger {

    // Private
    log_file: BufWriter <File>,
    log_to_console: bool
}

/*================================================================================================*/
/*------LOGGER PUBLIC STATIC METHODS--------------------------------------------------------------*/
/*================================================================================================*/

impl Logger {

    /// Initializes the logger
    ///
    /// This is required before any logging functions are performed.
    /// Any logging done beforehand will be ignored.
    ///
    /// # Arguments
    /// * `log_file_path` - The path to where the log will be saved
    /// * `log_to_console` - Whether the log output should also be printed to the console
    ///
    /// # Return value
    /// A result, returning an error on failure.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::logger::*;
    ///
    /// Logger::init ("LogFile.log", false);
    pub fn init (log_file_path: &str, log_to_console: bool) -> Result<(), log::SetLoggerError> {

        log::set_logger (|max_log_level| {

            if cfg! (debug_assertions) {
                max_log_level.set (log::LogLevelFilter::Debug);
            }

            else {
                max_log_level.set (log::LogLevelFilter::Info);
            }

            Box::new (Logger {log_file: BufWriter::new (File::create (&log_file_path).unwrap ()),
                              log_to_console: log_to_console})
        })
    }
}

/*================================================================================================*/
/*------LOGGER PRIVATE METHODS--------------------------------------------------------------------*/
/*================================================================================================*/

impl log::Log for Logger {

    fn enabled (&self, metadata: &log::LogMetadata) -> bool {

        if cfg! (debug_assertions) {
            return metadata.level () <= log::LogLevel::Debug;
        }

        metadata.level () <= log::LogLevel::Info
    }

/*================================================================================================*/

    fn log (&self, record: &log::LogRecord) {

        let string_output = format! ("{} ({} : {}) - {}\n", record.level (),
                                                            record.location ().file (),
                                                            record.location ().line (),
                                                            record.args ());
        if self.log_to_console {
            print! ("{}", string_output);
        }

        self.log_file.get_ref ().write (string_output.as_bytes ()).unwrap ();
    }
}

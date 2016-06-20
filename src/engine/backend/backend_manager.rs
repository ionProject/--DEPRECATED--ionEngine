use ::engine::backend::Config;

/*===============================================================================================*/
/*------MANAGER STRUCT---------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Manages the loading of the various backend systems.
#[derive (Default)]
pub struct Manager {

    // Public
    /// The backend configuration.
    pub config: Config,

    // Private
    _backend_list: Vec<String>,
    _ext: String,
}

/*===============================================================================================*/
/*------MANAGER PUBLIC METHODS-------------------------------------------------------------------*/
/*===============================================================================================*/

impl Manager {

    /// Queries the backend directory.
    ///
    ///
    pub fn query_backend_dir (&self) {
        
    }

/*===============================================================================================*/
/*------MANAGER PUBLIC STATIC METHODS------------------------------------------------------------*/
/*===============================================================================================*/

    /// Creates a new instance of the BackendManager.
    ///
    /// # Return value
    /// A new instance of the backend manager.
    pub fn new () -> Manager {

        Manager {

            config: Config::default (),
            _backend_list: Vec::new (),
            _ext: String::new ()
        }
    }
}

extern crate ion_core;

use self::ion_core::engine::App;

fn main () {

    let app = App::builder ()
        .project_name      ("ionSandbox")
        .project_developer ("ionProject")
        .build             ()
        .unwrap            ();

    app.init ();
    app.run  ();
    app.exit ();
}

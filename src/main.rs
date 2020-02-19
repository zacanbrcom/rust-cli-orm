extern crate cli_lib;

use clap::{clap_app, crate_version};
use diesel::prelude::*;

use cli_lib::{schema::*};
use cli_lib::models::{new_user, User};

fn main() -> Result<(), failure::Error> {
    println!("Start the CLI interface!");
    let clap = clap_app!(cli_test =>
                                    (about: "A CLI test to create an user on the db")
                                    (version: crate_version!())
                                    (author: "Andrea Zaccaro")
                                    (@subcommand new_user =>
                                        (@arg username: +required "The username to choose")
                                        (@arg password: +required "The new user password")
                                    )).get_matches();

    let conn = cli_lib::create_conn()?;
    if let Some(sub) = clap.subcommand_matches("new_user") {
        let user = new_user(
            sub.value_of("username").unwrap(),
            sub.value_of("password").unwrap(),
        );
        let added: User = diesel::insert_into(users::table).values(&user).get_result(&conn)?;
        println!("New user added {:?}", added);
    }
    Ok(())
}

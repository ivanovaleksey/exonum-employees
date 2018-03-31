extern crate diesel;
extern crate exonum_employees;
#[macro_use]
extern crate quicli;

use std::{env, process};

use diesel::PgConnection;
use diesel::prelude::*;
use exonum_employees::db_schema::superuser_keys;
use exonum_employees::superuser_key::{NewSuperuserKey, SuperuserKey};
use quicli::prelude::*;

/// Manage superuser keys
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
    verbosity: u8,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "add", about = "Adds a key to the storage")]
    Add { key: String },
    #[structopt(name = "ls", about = "Lists all keys in the storage")]
    List,
    #[structopt(name = "rm", about = "Remove a key from the storage")]
    Remove { key: String },
}

main!(|args: Cli, log_level: verbosity| {
    if let Err(err) = env::var("DATABASE_URL") {
        println!("DATABASE_URL {}", err);
        process::exit(1);
    }

    let conn = establish_connection();

    match args.cmd {
        Command::Add { key } => {
            let new_key = NewSuperuserKey { public_key: key };
            diesel::insert_into(superuser_keys::table)
                .values(&new_key)
                .execute(&conn)?;
        }
        Command::List => {
            let keys = superuser_keys::table.load::<SuperuserKey>(&conn)?;
            for key in &keys {
                println!("{}", key.public_key);
            }
        }
        Command::Remove { key } => {
            diesel::delete(superuser_keys::table.find(key))
                .execute(&conn)?;
        }
    }
});

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

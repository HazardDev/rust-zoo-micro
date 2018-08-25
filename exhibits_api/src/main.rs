#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]


#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate exhibits_svc;
extern crate animals_svc;

use animals_svc::models::*;
// use exhibits_svc::models::*;
use diesel::prelude::*;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn main() {

    rocket::ignite().mount("/animals", routes![get_animals, new_animal]).launch();
}

#[get("/")]
fn get_animals() -> Option<String> {
    use animals_svc::schema::animals::dsl::*;
    let connection = get_db_connection();
    match animals.limit(20).load::<Animal>(&connection) {
        Ok(ref items) if items.len() > 0 => {
            let mut return_string: String = "".to_string();

            for item in items {
                return_string.push_str(&format!("{},\n", item));
            }

            Some(return_string)
        },
        _ => {
            None
        }
    }
}

#[post("/<name>/<species>")]
fn new_animal(name: String, species: String) -> String {
    use animals_svc::birth_animal;

    let connection = get_db_connection();
    let animal = birth_animal(&connection, &name, &species);
    format!("{}", animal)
}


pub fn get_db_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

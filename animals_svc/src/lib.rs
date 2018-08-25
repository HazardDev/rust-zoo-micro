#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::{NewAnimal, Animal};

pub fn birth_animal<'a>(conn: &PgConnection, name: &'a str, species: &'a str) -> Animal {

    use schema::animals;

    let new_animal = NewAnimal {
        name: name,
        species: species,
    };

    diesel::insert_into(animals::table)
        .values(&new_animal)
        .get_result(conn)
        .expect("Error saving new animal")

}

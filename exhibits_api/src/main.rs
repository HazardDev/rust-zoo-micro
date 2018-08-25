#[macro_use]
extern crate diesel;
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
    use animals_svc::schema::animals::dsl::*;

    let connection = establish_connection();
    let results = animals.limit(1).load::<Animal>(&connection).expect("What!");
    // let results = posts.filter(published.eq(true))
    //     .limit(5)
    //     .load::<Animal>(&connection)
    //     .expect("Error loading posts");

    println!("Displaying {} animals", results.len());
    for animal in results {
        println!("Name: {}", animal.name);
        println!("----------\n");
        println!("Species: {}", animal.species);
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

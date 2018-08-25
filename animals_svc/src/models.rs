#[derive(Queryable)]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub species: String,
}

use std::fmt::{Display, Formatter, Error};
use std::result::Result;

impl Display for Animal {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[{}] {}: {}", self.species, self.id, self.name)
    }
}

use super::schema::animals;
#[derive(Insertable)]
#[table_name="animals"]
pub struct NewAnimal<'a> {
    pub name: &'a str,
    pub species: &'a str,
}

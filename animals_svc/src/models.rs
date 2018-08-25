#[derive(Queryable)]
pub struct Animal {
    pub id: i32,
    pub name: String,
    pub species: String,
}

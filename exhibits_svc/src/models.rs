use animals_svc::models::Animal;

#[derive(Queryable)]
pub struct Exhibit {
    pub id: i32,
    pub title: String,
    pub animals: Vec<Animal>,
    pub open: bool,
}

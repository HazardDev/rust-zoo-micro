table! {
    animals (id) {
        id -> Int4,
        name -> Varchar,
        species -> Varchar,
    }
}

table! {
    exhibits (id) {
        id -> Int4,
        name -> Varchar,
        animals -> Nullable<Array<Int4>>,
        open -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    animals,
    exhibits,
);

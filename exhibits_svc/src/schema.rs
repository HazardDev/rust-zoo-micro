table! {
    use diesel::sql_types::*;

    exhibits (id) {
        id -> Integer,
        name -> Text,
        animals -> Nullable<Array<Integer>>,
        open -> Bool,
    }
}

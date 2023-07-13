// @generated automatically by Diesel CLI.

diesel::table! {
    msgs (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}


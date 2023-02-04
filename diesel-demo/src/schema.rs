// @generated automatically by Diesel CLI.

diesel::table! {
 s   posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

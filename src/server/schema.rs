// @generated automatically by Diesel CLI.

diesel::table! {
    admin (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

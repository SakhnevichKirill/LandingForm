// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Nullable<Varchar>,
        phone_number_code -> Int4,
        phone_number -> Varchar,
        password -> Nullable<Varchar>,
        token -> Nullable<Varchar>,
    }
}

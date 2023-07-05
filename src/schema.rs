// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Int4,
        title -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Nullable<Varchar>,
        phone_number_code -> Int4,
        phone_number -> Varchar,
        password -> Nullable<Varchar>,
        token -> Nullable<Varchar>,
        verified -> Bool,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    users,
    users_roles,
);

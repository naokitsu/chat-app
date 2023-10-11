// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        salted_hash -> Nullable<Binary>,
    }
}

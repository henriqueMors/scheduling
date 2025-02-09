// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Uuid,
        name -> Text,
        phone -> Text,
        email -> Nullable<Text>,
    }
}

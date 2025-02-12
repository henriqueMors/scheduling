// @generated automatically by Diesel CLI.
diesel::table! {
    clients (id) {
        id -> Uuid,
        name -> Text,
        phone -> Text,
        email -> Nullable<Text>,
    }
}

diesel::table! {
    reservations (id) {
        id -> Uuid,
        client_id -> Uuid,
        datetime -> Timestamp,
    }
}

diesel::joinable!(reservations -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    reservations,
);

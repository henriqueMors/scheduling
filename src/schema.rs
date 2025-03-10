// @generated automatically by Diesel CLI.

diesel::joinable!(reservations -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    reservations,
    users,
);

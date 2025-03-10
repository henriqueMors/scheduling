// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        phone -> Text,
        password_hash -> Text,
        role -> Text,
        sms_verified -> Bool,
    }
}

diesel::table! {
    reservations (id) {
        id -> Uuid,
        user_id -> Uuid,
        service -> Text,
        appointment_time -> Timestamp,
        status -> Text,
    }
}

// 🔹 Define o relacionamento entre `reservations` e `users`
diesel::joinable!(reservations -> users (user_id));

// 🔹 Permite consultas combinadas entre `reservations` e `users`
diesel::allow_tables_to_appear_in_same_query!(
    reservations,
    users,
);

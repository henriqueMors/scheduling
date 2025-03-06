// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Uuid,
        master_id -> Uuid,
        name -> Text,
        phone -> Text,
        password_hash -> Text,
    }
}

diesel::table! {
    clients (id) {
        id -> Uuid,
        name -> Text,
        phone -> Text,
        email -> Nullable<Text>,
        user_id -> Uuid,
    }
}

diesel::table! {
    reservations (id) {
        id -> Uuid,
        client_id -> Uuid,
        service -> Text,
        appointment_time -> Timestamp,
        status -> Text,
    }
}

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

diesel::joinable!(clients -> users (user_id));
diesel::joinable!(reservations -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    clients,
    reservations,
    users,
);

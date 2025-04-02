// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Uuid,
        master_id -> Text,
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
    }
}

diesel::table! {
    professionals (id) {
        id -> Uuid,
        user_id -> Uuid,
        bio -> Nullable<Text>,
        specialties -> Nullable<Array<Nullable<Text>>>,
        created_at -> Timestamp,
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

diesel::table! {
    salon_settings (id) {
        id -> Uuid,
        opening_hour -> Time,
        closing_hour -> Time,
        working_days -> Array<Nullable<Text>>,
        created_at -> Timestamp,
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

diesel::table! {
    appointments (id) {
        id -> Uuid,
        client_id -> Uuid,           // Garantir que `client_id` seja do tipo `Uuid`
        professional_id -> Uuid,
        service_id -> Uuid,
        appointment_time -> Timestamp,
        status -> Text,
    }
}

diesel::joinable!(professionals -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    clients,
    professionals,
    reservations,
    salon_settings,
    users,
);

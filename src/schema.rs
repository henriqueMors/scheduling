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
    reservations (id) {
        id -> Uuid,
        user_id -> Uuid,
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

diesel::table! {
    salon_settings (id) {
        id -> Uuid,
        opening_hour -> Time,
        closing_hour -> Time,
        working_days -> Array<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    appointments (id) {
        id -> Uuid,
        client_id -> Uuid,
        professional_id -> Uuid,
        service_id -> Uuid,
        appointment_time -> Timestamp,
        status -> Text,
    }
}


diesel::table! {
    professionals (id) {
        id -> Uuid,
        name -> Text,
        phone -> Text,
        role -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    clients,
    reservations,
    users,
    salon_settings,
    appointments,
    professionals, // ✅ Agora a tabela `professionals` está incluída
);

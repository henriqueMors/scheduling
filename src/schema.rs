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
    availabilities (id) {
        id -> Uuid,
        professional_id -> Uuid,
        date -> Date,           // Deve ser do tipo Date no banco de dados
        start_time -> Time,     // Se o tipo for Time no banco de dados
        end_time -> Time,       // Se o tipo for Time no banco de dados
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
        opening_hour -> Time,  // No banco de dados, o tipo deve ser TIME para horários
        closing_hour -> Time,  // No banco de dados, o tipo deve ser TIME para horários
        working_days -> Array<Text>,  // Para um vetor de strings (ex: ["monday", "tuesday"])
        created_at -> Timestamp,  // Usando TIMESTAMP para a data de criação
    }
}
diesel::table! {
    services (id) {
        id -> Uuid,
        nome -> Text,
        descricao -> Nullable<Text>,
        preco -> Float8,
        duracao_min -> Int4,
        ativo -> Bool,
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
        created_at -> Timestamp,
    }
}

diesel::joinable!(appointments -> clients (client_id));
diesel::joinable!(appointments -> professionals (professional_id));
diesel::joinable!(appointments -> services (service_id));
diesel::joinable!(availabilities -> professionals (professional_id));
diesel::joinable!(professionals -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    appointments,
    availabilities,
    clients,
    professionals,
    reservations,
    salon_settings,
    services,
    users,
);

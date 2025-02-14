-- Your SQL goes here
CREATE TABLE reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id UUID NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    service TEXT NOT NULL DEFAULT 'General',
    appointment_time TIMESTAMP NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending'
);

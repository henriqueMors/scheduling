CREATE TABLE availabilities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    professional_id UUID NOT NULL REFERENCES professionals(id) ON DELETE CASCADE,
    available_time TIMESTAMP NOT NULL,
    status TEXT NOT NULL DEFAULT 'available'
);

CREATE TABLE salon_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    opening_hour TEXT NOT NULL,
    closing_hour TEXT NOT NULL,
    working_days JSONB NOT NULL, -- Usando JSONB para armazenar uma lista de dias
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE salon_settings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    opening_hour TIME NOT NULL,
    closing_hour TIME NOT NULL,
    working_days TEXT[] NOT NULL, -- Ex: ["monday", "tuesday", "wednesday"]
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- migration for services
CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,            -- Renomeado para `name`
    description TEXT,              -- Renomeado para `description`
    price DOUBLE PRECISION NOT NULL, -- Renomeado para `price`
    duration_minutes INTEGER NOT NULL,   -- Renomeado para `duration_minutes`
    is_active BOOLEAN NOT NULL DEFAULT TRUE -- Renomeado para `is_active`
);

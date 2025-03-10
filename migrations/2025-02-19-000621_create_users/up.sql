-- Cria a extensão para gerar UUIDs, se ainda não existir
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Cria a tabela de usuários
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    phone TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    
    -- Adiciona um valor padrão e uma restrição para os valores permitidos
    role TEXT NOT NULL DEFAULT 'client' CHECK (role IN ('client', 'admin', 'admin_master')),

    sms_verified BOOLEAN NOT NULL DEFAULT false
);

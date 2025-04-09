CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nome TEXT NOT NULL,                  -- Alterado para nome
    descricao TEXT,                      -- Alterado para descricao
    preco DOUBLE PRECISION NOT NULL,     -- Alterado para DOUBLE PRECISION
    duracao_min INTEGER NOT NULL,        -- Novo campo para duração em minutos
    ativo BOOLEAN NOT NULL DEFAULT TRUE  -- Novo campo para indicar se está ativo
);

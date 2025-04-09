-- Renomear a coluna 'price' para 'preco'
ALTER TABLE services RENAME COLUMN price TO preco;

-- Adiciona as novas colunas (duracao_min e ativo)
ALTER TABLE services ADD COLUMN duracao_min INTEGER NOT NULL DEFAULT 0;
ALTER TABLE services ADD COLUMN ativo BOOLEAN NOT NULL DEFAULT TRUE;

-- Modifica o tipo de dado da coluna 'preco' para DOUBLE PRECISION
ALTER TABLE services ALTER COLUMN preco TYPE DOUBLE PRECISION;

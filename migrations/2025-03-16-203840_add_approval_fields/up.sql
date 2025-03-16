-- Adiciona campo `status` e `role` na tabela `usuarios`
ALTER TABLE usuarios
ADD COLUMN role VARCHAR(20) CHECK (role IN ('admin_master', 'admin_aux', 'profissional', 'cliente')),
ADD COLUMN status VARCHAR(20) CHECK (status IN ('pendente', 'ativo', 'reprovado'));

-- Criação da tabela `profissionais`
CREATE TABLE profissionais (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    usuario_id UUID NOT NULL,
    data_contratacao DATE NOT NULL,
    informacoes_adicionais TEXT NOT NULL,
    CONSTRAINT fk_usuario_id FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE
);

-- Criação da tabela `especialidades`
CREATE TABLE especialidades (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(100) NOT NULL
);

-- Criação da tabela de associação entre profissionais e especialidades
CREATE TABLE profissional_especialidade (
    profissional_id UUID NOT NULL,
    especialidade_id INT NOT NULL,
    CONSTRAINT fk_profissional_id FOREIGN KEY (profissional_id) REFERENCES profissionais(id) ON DELETE CASCADE,
    CONSTRAINT fk_especialidade_id FOREIGN KEY (especialidade_id) REFERENCES especialidades(id) ON DELETE CASCADE
);

-- Criação da tabela `reservation_logs`
CREATE TABLE reservation_logs (
    id SERIAL PRIMARY KEY,
    reservation_id UUID NOT NULL,
    action VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    CONSTRAINT fk_reservation_id FOREIGN KEY (reservation_id) REFERENCES agendamentos(id) ON DELETE CASCADE
);

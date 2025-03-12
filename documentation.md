# Scheduling API

Este é um projeto de **API para agendamento de serviços** utilizando **Axum** (framework Rust) e 
**Diesel** (ORM para banco de dados PostgreSQL). O sistema permite que os usuários se registrem, 
façam login, criem reservas e gerenciem as suas. O sistema também implementa autenticação JWT e 
controle de acesso baseado em roles (admin, client).

## Tecnologias Utilizadas

- **Axum**: Framework web assíncrono em Rust.
- **Diesel**: ORM para comunicação com o banco de dados PostgreSQL.
- **JWT**: Autenticação com tokens JWT para garantir a segurança das rotas protegidas.
- **PostgreSQL**: Banco de dados relacional utilizado para persistência dos dados.
- **tracing**: Para logs e monitoramento da aplicação.

## Funcionalidades Principais

- **Cadastro de usuários**
- **Login e geração de JWT**
- **Criação, listagem e modificação de reservas**
- **Controle de rate limiting e autenticação via middleware**
- **Gestão de CORS**
   
## Configuração do Ambiente

### Requisitos

- **Rust** (com Cargo) versão >= 1.56
- **PostgreSQL** 13+
- **Docker** (verificando possibilidade)

### Passos para Configuração

1. **Clonar o Repositório**:

```bash```
git clone https://github.com/seu-usuario/scheduling.git
cd scheduling

Crie um arquivo .env na raiz do projeto com as seguintes variáveis
DATABASE_URL=postgres://username:password@localhost/scheduling
JWT_SECRET_KEY=seu_secreto_aqui

Se você não está usando Docker para o PostgreSQL, certifique-se de que o banco de dados PostgreSQL está rodando e crie o banco
psql -U seu_usuario -d postgres
CREATE DATABASE scheduling;

Rodar as Migrações
cargo diesel migration run

Rodar o Servidor (O servidor estará disponível em http://127.0.0.1:3000)
cargo run --quiet


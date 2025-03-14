# Scheduling API

src/
├── config/                # Configurações globais (Ex: leitura de variáveis de ambiente)
│   └── mod.rs
├── db/                    # Lógica de conexão com banco de dados (Diesel)
│   └── mod.rs
├── handlers/              # Funções de manipulação de requisições (Handlers)
│   ├── auth.rs            # Manipulação de autenticação (login, registro, etc.)
│   ├── reservation.rs     # Manipulação das reservas (CRUD)
│   └── user.rs            # Manipulação de usuários (CRUD)
├── middleware/            # Middleware, como autenticação, rate limit, etc.
│   ├── auth_middleware.rs # Verificação de JWT e roles
│   ├── rate_limit.rs      # Controle de rate limit
│   └── cors.rs            # Middleware de CORS
├── models/                # Modelos de dados (estruturas usadas para comunicação com o DB)
│   ├── user.rs            # Modelo de dados de usuários
│   └── reservation.rs     # Modelo de dados de reservas
├── routes/                # Definição de rotas
│   └── mod.rs             # Definição das rotas principais
├── schema/                # Esquema do banco de dados (gerado pelo Diesel)
│   └── mod.rs
├── services/              # Lógica de negócios, como validação, cálculos, etc.
│   └── mod.rs
├── utils/                 # Funções auxiliares
│   └── mod.rs
├── main.rs                # Ponto de entrada do aplicativo
└── lib.rs                 # (Opcional) Definições de utilidades para uso em testes ou outras partes



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

**Clonar o Repositório**:

```bash```
git clone https://github.com/seu-usuario/scheduling.git
cd scheduling

Crie um arquivo .env na raiz do projeto com as seguintes variáveis:
DATABASE_URL=postgres://username:password@localhost/scheduling
JWT_SECRET_KEY=seu_secreto_aqui

Se você não está usando Docker para o PostgreSQL, certifique-se de que o banco de dados PostgreSQL está rodando e crie o banco
psql -U seu_usuario -d postgres
CREATE DATABASE scheduling;

Rodar as Migrações
cargo diesel migration run

Rodar o Servidor (O servidor estará disponível em http://127.0.0.1:3000)
cargo run --quiet

Criar o Banco de Dado

psql -U seu_usuario -d postgres
CREATE DATABASE scheduling;

Rodar as Migrações
cargo diesel migration run

Rodar o Servidor
cargo run

## Endpoints da API

### 1. **POST /auth/register**
- **Descrição**: Cadastra um novo usuário no sistema.
- **Parâmetros**:
  - `username`: Nome de usuário (String)
  - `password`: Senha do usuário (String)
- **Exemplo de Requisição**:
  
```json
{
  "username": "johndoe",
  "password": "securepassword123"
}

Resposta:
Status: 201 Created
corpo:
{
  "id": "uuid-do-usuario",
  "username": "johndoe"
}

POST /auth/login
Descrição: Faz o login e retorna um token JWT.
Parâmetros:
username: Nome de usuário (String)
password: Senha (String)
Exemplo de Requisição:
{
  "username": "johndoe",
  "password": "securepassword123"
}
Resposta:
Status: 200 OK
Corpo:
{
  "access_token": "jwt_token_aqui"
}

3. GET /health
Descrição: Verifica se o serviço está em funcionamento.
Resposta:
Status: 200 OK
Corpo:
{
  "message": "Service is running!"
}

. POST /reservations
Descrição: Cria uma nova reserva.
Parâmetros:
service: Nome do serviço (String)
appointment_time: Data e hora do agendamento (ISO 8601)
Exemplo de Requisição:
{
  "service": "Consulta médica",
  "appointment_time": "2023-12-01T10:00:00"
}
Resposta:
Status: 201 Created
Corpo:
{
  "id": "uuid-da-reserva",
  "user_id": "uuid-do-usuario",
  "service": "Consulta médica",
  "appointment_time": "2023-12-01T10:00:00",
  "status": "pending"
}
GET /reservations
Descrição: Lista todas as reservas para o usuário autenticado.
Parâmetros: Nenhum
Resposta:
Status: 200 OK
Corpo:
[
  {
    "id": "uuid-da-reserva",
    "user_id": "uuid-do-usuario",
    "service": "Consulta médica",
    "appointment_time": "2023-12-01T10:00:00",
    "status": "pending"
  }
]



### 4. **Autenticação e Autorização**

O sistema usa **JWT** para autenticação e o middleware é utilizado para validar tokens em rotas protegidas.

```markdown
## Autenticação

As rotas protegidas utilizam **JWT** para autenticação. O token é gerado ao fazer login no sistema e deve ser enviado no cabeçalho `Authorization` com o prefixo `Bearer`.

### Exemplo de Cabeçalho de Autenticação:

```plaintext
Authorization: Bearer seu_token_aqui

Middleware de Autenticação
O middleware de autenticação verifica se o token é válido e não expirou. Se o token for válido, ele permite o acesso à rota. Caso contrário, retorna um erro 401 (Unauthorized).


### 5. **Estrutura do Banco de Dados**

```markdown
## Banco de Dados

### Tabelas

1. **users**: Contém os dados dos usuários (id, username, password_hash).
2. **reservations**: Contém as reservas feitas pelos usuários (id, user_id, service, appointment_time, status).

### Mapeamento Diesel

A tabela `reservations` é mapeada para o modelo `Reservation`, onde o `user_id` faz referência à tabela `users` (com chave estrangeira).

## Testes

Os testes são realizados com `cargo test`. Para garantir que o sistema funciona corretamente, os seguintes testes foram implementados:

1. **Testes de Endpoints**: Verificam se as respostas da API estão corretas.
2. **Testes de Validação de JWT**: Verificam se o middleware de autenticação está funcionando corretamente.
3. **Testes de Rate-Limiting**: Garantem que o middleware de rate-limiting está funcionando.

### Rodando os Testes

```bash
cargo test



### 7. **Próximos Passos**

Liste as tarefas que precisam ser realizadas ainda, como adicionar novos endpoints, melhorar a segurança, etc.

```markdown
## Próximos Passos

- Adicionar mais funcionalidades, como o gerenciamento de roles de usuários (admin, cliente).
- Implementar mais testes automatizados.
- Implementar logs mais detalhados para produção.
- Melhorar a documentação da API com Swagger/OpenAPI.

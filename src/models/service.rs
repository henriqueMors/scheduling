use diesel::{Queryable, Insertable, Identifiable, AsChangeset, Selectable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::services;

/// 🔹 Estrutura de Serviço (Tabela `services`)
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Selectable)]
#[diesel(table_name = services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Service {
    pub id: Uuid,                  // ID único do serviço
    pub nome: String,              // Nome do serviço (renomeado de 'name' para 'nome')
    pub descricao: Option<String>, // Descrição do serviço (renomeado de 'description' para 'descricao')
    pub preco: f64,                // Preço do serviço
    pub duracao_min: i32,          // Duração do serviço em minutos
    pub ativo: bool,               // Indica se o serviço está ativo ou não
}

/// 🔹 Estrutura para criar um novo serviço (para inserção no banco)
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = services)]
pub struct NewService {
    pub nome: String,              // Nome do serviço
    pub descricao: Option<String>, // Descrição do serviço (renomeado de 'description' para 'descricao')
    pub preco: f64,                // Preço do serviço
    pub duracao_min: i32,          // Duração do serviço em minutos
    pub ativo: bool,               // Indica se o serviço está ativo ou não
}

/// 🔹 Estrutura para atualizar um serviço existente (para atualização no banco)
#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = services)]
pub struct UpdateService {
    pub nome: Option<String>,      // Nome do serviço (opcional para atualização)
    pub descricao: Option<String>, // Descrição do serviço (opcional para atualização)
    pub preco: Option<f64>,        // Preço do serviço (opcional para atualização)
    pub duracao_min: Option<i32>,  // Duração do serviço em minutos (opcional para atualização)
    pub ativo: Option<bool>,       // Status de ativação do serviço (opcional para atualização)
}

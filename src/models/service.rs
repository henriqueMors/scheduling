use diesel::{Queryable, Insertable, Identifiable, AsChangeset, Selectable};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::schema::services;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Selectable)]
#[diesel(table_name = services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Service {
    pub id: Uuid,
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: f64,
    pub duracao_min: i32,
    pub ativo: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = services)]
pub struct NewService {
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: f64,
    pub duracao_min: i32,
    pub ativo: bool,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = services)]
pub struct UpdateService {
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub preco: Option<f64>,
    pub duracao_min: Option<i32>,
    pub ativo: Option<bool>,
}

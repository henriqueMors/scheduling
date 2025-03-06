use diesel::prelude::*;
use uuid::Uuid;
use diesel::result::Error;
use crate::models::client::{Client, NewClient, UpdateClient};
use crate::schema::clients::dsl::*; // ✅ Usa `clients::dsl::*` para evitar referências desnecessárias

/// Cria um novo cliente e retorna o registro criado.
pub fn create_client(conn: &mut PgConnection, new_client: NewClient) -> Result<Client, Error> {
    diesel::insert_into(clients)
        .values(&new_client)
        .get_result(conn)
}

/// Busca um cliente pelo ID.
pub fn get_client_by_id(conn: &mut PgConnection, client_id: Uuid) -> Result<Client, Error> {
    clients.filter(id.eq(client_id)).first(conn)
}

/// Retorna a lista de todos os clientes.
pub fn get_all_clients(conn: &mut PgConnection) -> Result<Vec<Client>, Error> {
    clients.load(conn)
}

/// Atualiza os dados de um cliente identificado pelo ID.
pub fn update_client(conn: &mut PgConnection, client_id: Uuid, updated_data: UpdateClient) -> Result<Client, Error> {
    diesel::update(clients.filter(id.eq(client_id))) // ✅ Usa `filter(id.eq(client_id))`
        .set(&updated_data)
        .get_result(conn)
}

/// Deleta um cliente identificado pelo ID.
pub fn delete_client(conn: &mut PgConnection, client_id: Uuid) -> Result<usize, Error> {
    diesel::delete(clients.filter(id.eq(client_id))) // ✅ Mesmo ajuste para `delete_client`
        .execute(conn)
}

use diesel::prelude::*;
use uuid::Uuid;
use crate::models::reservation::{Reservation, NewReservation, UpdateReservation};
use crate::schema::reservations::dsl::*;
use diesel::result::Error;

/// Lista todas as reservas.
pub fn list_reservations(conn: &mut PgConnection) -> Result<Vec<Reservation>, Error> {
    reservations.load::<Reservation>(conn)
}

/// Cria uma nova reserva.
pub fn create_reservation(conn: &mut PgConnection, new_reservation: NewReservation) -> Result<Reservation, Error> {
    diesel::insert_into(reservations)
        .values(&new_reservation)
        .get_result(conn)
}

/// Busca uma reserva pelo ID.
pub fn get_reservation_by_id(conn: &mut PgConnection, reservation_id: Uuid) -> Result<Reservation, Error> {
    reservations.filter(id.eq(reservation_id))
        .first(conn)
}

/// Atualiza uma reserva existente.
pub fn update_reservation(conn: &mut PgConnection, reservation_id: Uuid, update: UpdateReservation) -> Result<Reservation, Error> {
    diesel::update(reservations.filter(id.eq(reservation_id)))
        .set(&update)
        .get_result(conn)
}

/// Deleta uma reserva pelo ID.
pub fn delete_reservation(conn: &mut PgConnection, reservation_id: Uuid) -> Result<usize, Error> {
    diesel::delete(reservations.filter(id.eq(reservation_id)))
        .execute(conn)
}

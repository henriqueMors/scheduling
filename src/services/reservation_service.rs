use diesel::prelude::*;
use uuid::Uuid;
use crate::models::reservation::{Reservation, NewReservation, UpdateReservation};
use crate::schema::reservations::dsl::*;
use diesel::result::Error;

/// ✅ Retorna todas as reservas.
pub fn list_reservations(conn: &mut PgConnection) -> Result<Vec<Reservation>, Error> {
    reservations
        .order(appointment_time.asc()) // ✅ Ordena por data/hora
        .load::<Reservation>(conn)
}

/// ✅ Busca uma reserva por ID.
pub fn get_reservation_by_id(conn: &mut PgConnection, reservation_id: Uuid) -> Result<Reservation, Error> {
    reservations
        .filter(id.eq(reservation_id))
        .first::<Reservation>(conn)
}

/// ✅ Cria uma nova reserva.
pub fn create_reservation(conn: &mut PgConnection, new_reservation: NewReservation) -> Result<Reservation, Error> {
    diesel::insert_into(reservations)
        .values(&new_reservation)
        .get_result::<Reservation>(conn)
}

/// ✅ Atualiza uma reserva existente.
pub fn update_reservation(
    conn: &mut PgConnection,
    reservation_id: Uuid,
    update: UpdateReservation,
) -> Result<Reservation, Error> {
    diesel::update(reservations.filter(id.eq(reservation_id)))
        .set(&update)
        .get_result::<Reservation>(conn)
}

/// ✅ Deleta uma reserva.
pub fn delete_reservation(conn: &mut PgConnection, reservation_id: Uuid) -> Result<usize, Error> {
    diesel::delete(reservations.filter(id.eq(reservation_id)))
        .execute(conn)
}

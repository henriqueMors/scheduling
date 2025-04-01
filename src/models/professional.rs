use diesel::{Queryable, Insertable, Identifiable, Selectable, Associations, AsChangeset};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::professionals;

#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable, Associations, Selectable)]
#[diesel(table_name = professionals)]
#[diesel(belongs_to(User))]
pub struct Professional {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub specialties: Vec<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = professionals)]
pub struct NewProfessional {
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub specialties: Vec<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = professionals)]
pub struct UpdateProfessional {
    pub bio: Option<String>,
    pub specialties: Option<Vec<String>>,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,      // "client", "admin" ou "admin_master"
    pub sms_verified: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub role: String,       // "client", "admin" ou "admin_master"
    pub sms_verified: bool,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<String>,
    pub sms_verified: Option<bool>,
}

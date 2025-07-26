use crate::schema::users;
use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
    pub verification_token: Option<String>,
    pub verification_token_expires: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires: Option<NaiveDateTime>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
}

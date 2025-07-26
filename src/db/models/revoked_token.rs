use crate::schema::revoked_tokens;
use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = revoked_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RevokedToken {
    pub id: Uuid,
    pub token_jti: String,
    pub user_id: Uuid,
    pub expiry: NaiveDateTime,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = revoked_tokens)]
pub struct NewRevokedToken {
    pub token_jti: String,
    pub user_id: Uuid,
    pub expiry: NaiveDateTime,
}

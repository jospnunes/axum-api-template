use crate::db::models::{
    refresh_token::{RefreshToken, NewRefreshToken},
};
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_refresh_token(
    new_token: &NewRefreshToken,
    conn: &mut PgConnection,
) -> QueryResult<RefreshToken> {
    use crate::schema::refresh_tokens::dsl::*;

    diesel::insert_into(refresh_tokens)
        .values(new_token)
        .returning(RefreshToken::as_returning())
        .get_result(conn)
}

pub fn find_refresh_token(
    token_value: &str,
    conn: &mut PgConnection,
) -> QueryResult<RefreshToken> {
    use crate::schema::refresh_tokens::dsl::*;

    refresh_tokens
        .filter(token.eq(token_value))
        .filter(expires_at.gt(chrono::Utc::now().naive_utc()))
        .select(RefreshToken::as_select())
        .first(conn)
}

pub fn delete_refresh_token(
    token_value: &str,
    conn: &mut PgConnection,
) -> QueryResult<usize> {
    use crate::schema::refresh_tokens::dsl::*;

    diesel::delete(refresh_tokens.filter(token.eq(token_value)))
        .execute(conn)
}

pub fn delete_user_refresh_tokens(
    user_id_val: Uuid,
    conn: &mut PgConnection,
) -> QueryResult<usize> {
    use crate::schema::refresh_tokens::dsl::*;

    diesel::delete(refresh_tokens.filter(user_id.eq(user_id_val)))
        .execute(conn)
}



pub fn is_token_revoked(
    token_jti_value: &str,
    conn: &mut PgConnection,
) -> QueryResult<bool> {
    use crate::schema::revoked_tokens::dsl::*;

    let count: i64 = revoked_tokens
        .filter(token_jti.eq(token_jti_value))
        .filter(expiry.gt(chrono::Utc::now().naive_utc()))
        .count()
        .get_result(conn)?;

    Ok(count > 0)
}

pub fn clean_expired_refresh_tokens(conn: &mut PgConnection) -> QueryResult<usize> {
    use crate::schema::refresh_tokens::dsl::*;

    diesel::delete(refresh_tokens.filter(expires_at.lt(chrono::Utc::now().naive_utc())))
        .execute(conn)
}

pub fn clean_expired_revoked_tokens(conn: &mut PgConnection) -> QueryResult<usize> {
    use crate::schema::revoked_tokens::dsl::*;

    diesel::delete(revoked_tokens.filter(expiry.lt(chrono::Utc::now().naive_utc())))
        .execute(conn)
}

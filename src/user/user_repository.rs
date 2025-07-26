use crate::db::models::user::{User, NewUser};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_user(new_user: &NewUser, conn: &mut PgConnection) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn find_user_by_id(user_id: Uuid, conn: &mut PgConnection) -> QueryResult<User> {
    users
        .filter(id.eq(user_id))
        .filter(is_active.eq(true))
        .select(User::as_select())
        .first(conn)
}

pub fn find_user_by_email(user_email: &str, conn: &mut PgConnection) -> QueryResult<User> {
    users
        .filter(email.eq(user_email))
        .filter(is_active.eq(true))
        .select(User::as_select())
        .first(conn)
}

pub fn update_user(user_id: Uuid, user_data: &User, conn: &mut PgConnection) -> QueryResult<User> {
    diesel::update(users.filter(id.eq(user_id)))
        .set(user_data)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn find_user_by_reset_token(token: &str, conn: &mut PgConnection) -> QueryResult<User> {
    users
        .filter(password_reset_token.eq(token))
        .filter(is_active.eq(true))
        .select(User::as_select())
        .first(conn)
}

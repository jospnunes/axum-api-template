// @generated automatically by Diesel CLI.

diesel::table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        expires_at -> Timestamp,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    revoked_tokens (id) {
        id -> Uuid,
        token_jti -> Text,
        user_id -> Uuid,
        expiry -> Timestamp,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        password_hash -> Text,
        is_active -> Nullable<Bool>,
        is_verified -> Nullable<Bool>,
        verification_token -> Nullable<Text>,
        verification_token_expires -> Nullable<Timestamp>,
        password_reset_token -> Nullable<Text>,
        password_reset_expires -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(revoked_tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    refresh_tokens,
    revoked_tokens,
    users,
);

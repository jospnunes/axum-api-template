DROP INDEX IF EXISTS idx_revoked_tokens_expiry;
DROP INDEX IF EXISTS idx_revoked_tokens_token_jti;
DROP INDEX IF EXISTS idx_refresh_tokens_expires_at;
DROP INDEX IF EXISTS idx_refresh_tokens_token;
DROP INDEX IF EXISTS idx_refresh_tokens_user_id;
DROP TABLE IF EXISTS revoked_tokens;
DROP TABLE IF EXISTS refresh_tokens;

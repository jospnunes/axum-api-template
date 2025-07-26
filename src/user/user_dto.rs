use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserProfileRequest {
    #[validate(length(min = 1, max = 255, message = "First name must be between 1 and 255 characters"))]
    pub first_name: Option<String>,

    #[validate(length(min = 1, max = 255, message = "Last name must be between 1 and 255 characters"))]
    pub last_name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

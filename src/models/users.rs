use serde::{Deserialize, Serialize};

use super::errors::ApiError;

#[derive(Debug, Serialize)]
pub struct UserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    email: String,
    password: String,
}

impl UserRequest {
    pub fn new(username: Option<String>, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthRequest {
    pub user: UserRequest,
}

impl AuthRequest {
    pub fn new(username: Option<String>, email: String, password: String) -> Self {
        Self {
            user: UserRequest::new(username, email, password),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthResponseContext {
    AuthenticatedUser(User),
    ValidationError(ApiError),
}

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub user: User,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
}

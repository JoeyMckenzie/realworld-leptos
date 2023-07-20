use http::{
    header::{ACCEPT, CONTENT_ENCODING},
    HeaderMap, StatusCode,
};

use crate::{
    error_template::AppError,
    models::{
        errors::ApiError,
        users::{AuthRequest, AuthResponse, AuthResponseContext},
    },
};

use super::API_BASE_URL;

#[derive(Debug, Clone, Copy)]
pub struct UsersService;

impl Default for UsersService {
    fn default() -> Self {
        Self::new()
    }
}

impl UsersService {
    pub fn new() -> Self {
        Self
    }

    fn get_client(&self) -> reqwest::Client {
        let mut headers = HeaderMap::new();
        headers.append(CONTENT_ENCODING, "application/json".parse().unwrap());
        headers.append(ACCEPT, "application/json".parse().unwrap());
        reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap()
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<AuthResponseContext, AppError> {
        leptos::log!("registering user {}", email);

        let request = AuthRequest::new(Some(username), email, password);
        let response = self
            .get_client()
            .post(format!("{}/users", API_BASE_URL))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                leptos::log!("user successfully register");
                let user = response.json::<AuthResponse>().await?;
                Ok(AuthResponseContext::AuthenticatedUser(user.user))
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                leptos::log!("failed to register user");
                let validation_errors = response.json::<ApiError>().await?;
                leptos::log!("validation failures: {:?}", validation_errors);
                Ok(AuthResponseContext::ValidationError(validation_errors))
            }
            _ => Err(AppError::InternalError),
        }
    }
}

use http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderMap, StatusCode,
};

use crate::{
    error_template::AppError,
    models::{
        errors::ApiError,
        users::{AuthRequest, AuthResponse, User},
    },
};

use super::{API_BASE_URL, JSON_ENCODING_MEDIA_VALUE};

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
        headers.append(CONTENT_TYPE, JSON_ENCODING_MEDIA_VALUE.parse().unwrap());
        headers.append(ACCEPT, JSON_ENCODING_MEDIA_VALUE.parse().unwrap());
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
    ) -> Result<User, AppError> {
        leptos::log!("registering user {}", email);

        let request = AuthRequest::new(Some(username), email, password);
        let response = self
            .get_client()
            .post(format!("{}/users", API_BASE_URL))
            .json(&request)
            .send()
            .await?;

        match response.status() {
            StatusCode::CREATED => {
                leptos::log!("user successfully registered");
                let user = response.json::<AuthResponse>().await?;
                Ok(user.user)
            }
            StatusCode::UNPROCESSABLE_ENTITY => {
                let validation_errors = response.json::<ApiError>().await?;
                leptos::log!(
                    "failed to register user, validation failures: {:?}",
                    validation_errors
                );
                Err(AppError::ValidationFailed(validation_errors))
            }
            _ => {
                let errors = response.json::<serde_json::Value>().await?;
                leptos::log!("unexpected error occurred: {:?}", errors);
                Err(AppError::InternalError)
            }
        }
    }
}
